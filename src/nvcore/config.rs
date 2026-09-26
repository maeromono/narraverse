use std::{fs::{File, create_dir_all, read_to_string, rename}, io::{Error, ErrorKind, Result, Write}, path::PathBuf};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use toml::{from_str, to_string};

#[derive(Serialize, Deserialize)]
pub struct NarraverseConfig {
    pub default_written_by: String,
}

impl Default for NarraverseConfig {
    fn default() -> Self {
        Self {
            default_written_by: String::from("Me"),
        }
    }
}

fn save_path() -> Result<PathBuf> {
    let dirs = ProjectDirs::from("org.neocities", "enmkdev", "narraverse")
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "cannot locate data directory"))?;
    Ok(dirs.config_local_dir().join("config.toml"))
}

pub fn save_config(config: &NarraverseConfig) -> Result<()> {
    let path = save_path()?;
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    let contents = to_string(config).map_err(|err| Error::new(ErrorKind::InvalidData, err))?;
    let tmp_path = path.with_extension("tmp");
    let mut tmp = File::create(&tmp_path)?;
    tmp.write_all(contents.as_bytes())?;
    tmp.sync_all()?;
    rename(&tmp_path, &path)?;
    Ok(())
}

pub fn load_config() -> Result<NarraverseConfig> {
    let path = save_path()?;
    let contents = match read_to_string(&path) {
        Ok(contents) => contents,
        Err(err) if err.kind() == ErrorKind::NotFound => return Ok(NarraverseConfig::default()),
        Err(err) => return Err(err),
    };
    match from_str(&contents) {
        Ok(config) => Ok(config),
        Err(err) => {
            let _ = rename(&path, &path.with_extension("corrupt"));
            Err(Error::new(ErrorKind::InvalidData, err))
        }
    }
}
