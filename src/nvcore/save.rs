use std::{
    fs::{File, create_dir_all, read_to_string, rename},
    io::{Error, ErrorKind, Result, Write},
    path::{Path, PathBuf},
};

use directories::ProjectDirs;
use indexmap::IndexMap;
use toml::{from_str, to_string};
use uuid::Uuid;

use crate::nvcore::verses::{NarraverseVerses, Verse};

fn save_path(file: &str) -> Result<PathBuf> {
    let dirs = ProjectDirs::from("sh.surge", "maero", "narraverse")
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "cannot locate data directory"))?;
    Ok(dirs.data_dir().join(format!("{}.toml", file)))
}

pub fn save_manifest(nv: &mut NarraverseVerses) -> Result<()> {
    let path = save_path("narraverse")?;
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    let contents = to_string(nv).map_err(|err| Error::new(ErrorKind::InvalidData, err))?;
    write(&path, &contents)?;
    Ok(())
}

pub fn save_verses(nv: &mut NarraverseVerses) -> Result<()> {
    let mut verse_files: IndexMap<Uuid, String> = IndexMap::default();
    for (uuid, verse) in &mut nv.verses {
        let filename = format!("verse-{}", uuid);
        verse_files.insert(*uuid, filename.clone());
        let path = save_path(filename.as_str())?;
        let contents = to_string(&verse).map_err(|err| Error::new(ErrorKind::InvalidData, err))?;
        write(&path, &contents)?;
    }
    nv.verse_files = verse_files;
    save_manifest(nv)?;
    Ok(())
}

pub fn save_verse(verse: &mut Verse, file: &str) -> Result<()> {
    let path = save_path(file)?;
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    let contents = to_string(verse).map_err(|err| Error::new(ErrorKind::InvalidData, err))?;
    write(&path, &contents)?;
    Ok(())
}

fn write(path: &Path, contents: &str) -> Result<()> {
    if let Ok(existing) = read_to_string(path) {
        if existing == contents {
            return Ok(());
        }
    }
    let tmp_path = path.with_extension("tmp");
    let mut tmp = File::create(&tmp_path)?;
    tmp.write_all(contents.as_bytes())?;
    tmp.sync_all()?;
    rename(&tmp_path, &path)?;
    Ok(())
}

pub fn load_verses() -> Result<NarraverseVerses> {
    let path = save_path("narraverse")?;
    let contents = match read_to_string(&path) {
        Ok(contents) => contents,
        Err(err) if err.kind() == ErrorKind::NotFound => return Ok(NarraverseVerses::default()),
        Err(err) => return Err(err),
    };
    let mut nv: NarraverseVerses = match from_str(&contents) {
        Ok(nv) => Ok(nv),
        Err(err) => {
            let _ = rename(&path, &path.with_extension("corrupt"));
            Err(Error::new(ErrorKind::InvalidData, err))
        }
    }?;
    for file in &nv.verse_files {
        let path = save_path(file.1)?;
        let contents = match read_to_string(&path) {
            Ok(contents) => contents,
            Err(err) if err.kind() == ErrorKind::NotFound => continue,
            Err(err) => return Err(err),
        };
        let verse_itself: Verse = match from_str(&contents) {
            Ok(verse) => verse,
            Err(_) => {
                let _ = rename(&path, &path.with_extension("corrupt"));
                continue;
            }
        };
        nv.verses.insert(*file.0, verse_itself);
    }
    let verses_len = nv.verses.len();
    if verses_len != nv.verse_files.len() {
        nv.verse_files.truncate(verses_len);
    }
    Ok(nv)
}
