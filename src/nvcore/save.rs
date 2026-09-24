use std::{
    fs::{File, create_dir_all, read_to_string, rename},
    io::{Error, ErrorKind, Result, Write},
    path::PathBuf,
};

use directories::ProjectDirs;
use toml::{from_str, to_string};

use crate::nvcore::verses::NarraverseVerses;

fn save_path(file: &str) -> Result<PathBuf> {
    let dirs = ProjectDirs::from("org.neocities", "enmkdev", "narraverse")
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "cannot locate data directory"))?;
    Ok(dirs.data_dir().join(format!("{}.toml", file)))
}

pub fn save_verses(nv: &NarraverseVerses) -> Result<()> {
    let path = save_path("narraverse")?;
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    let contents = to_string(nv).map_err(|err| Error::new(ErrorKind::InvalidData, err))?;
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
    match from_str(&contents) {
        Ok(av) => Ok(av),
        Err(err) => {
            let _ = rename(&path, &path.with_extension("corrupt"));
            Err(Error::new(ErrorKind::InvalidData, err))
        }
    }
}
