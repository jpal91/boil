use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use crate::error::{BoilResult, BoilError};

pub fn default_config() -> BoilResult<PathBuf> {
    if let Ok(p) = env::var("BOIL_DEF_CONFIG") {
        let path: PathBuf = PathBuf::from(&p);

        Ok(path)
    } else if let Some(home) = dirs::config_dir() {
        let path: PathBuf = [home.as_path(), Path::new(".boil/config.toml")].iter().collect();

        Ok(path)
    } else {
        Err(BoilError::ConfigCreate)
    }
}

pub fn default_proj_path() -> PathBuf {
    if let Ok(p) = env::var("BOIL_PROJ_PATH") {
        let path: PathBuf = PathBuf::from(&p);

        path
    } else if let Some(home) = dirs::home_dir() {
        let path: PathBuf = PathBuf::from_iter([home.as_path(), Path::new("dev")]);

        path
    } else {
        panic!()
    }
}

pub fn default_bin_path() -> PathBuf {
    if let Some(home) = dirs::home_dir() {
        let path: PathBuf = PathBuf::from_iter([home.as_path(), Path::new("bin")]);

        path
    } else {
        panic!()
    }
}