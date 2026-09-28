use std::{
    fs::{File, create_dir_all, read_to_string, rename},
    io::{Error, ErrorKind, Result, Write},
    path::{Path, PathBuf},
};

use directories::ProjectDirs;
use toml::{from_str, to_string};

use crate::nvcore::verses::{NarraverseVerses, Verse};

fn save_path(file: &str) -> Result<PathBuf> {
    let dirs = ProjectDirs::from("org.neocities", "enmkdev", "narraverse")
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "cannot locate data directory"))?;
    Ok(dirs.data_dir().join(format!("{}.toml", file)))
}

pub fn save_verses(nv: &mut NarraverseVerses) -> Result<()> {
    let path = save_path("narraverse")?;
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    nv.refresh_verse_files();
    for (verse, file) in nv.verses.iter().zip(nv.verse_files.iter()) {
        let path = save_path(file.as_str())?;
        let contents = to_string(verse).map_err(|err| Error::new(ErrorKind::InvalidData, err))?;
        write(&path, &contents)?;
    }
    let contents = to_string(nv).map_err(|err| Error::new(ErrorKind::InvalidData, err))?;
    write(&path, &contents)?;
    Ok(())
}

fn write(path: &Path, contents: &str) -> Result<()> {
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
        let path = save_path(file)?;
        if !Path::new(&path).try_exists().unwrap_or(false) {
            continue;
        }
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
        nv.verses.push(verse_itself);
    }
    Ok(nv)
}
