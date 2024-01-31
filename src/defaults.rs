use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use crate::error::{BoilResult, BoilError};

pub fn default_config() -> BoilResult<PathBuf> {
    if let Ok(p) = env::var("BOIL_DEF_CONFIG") {
        let path: PathBuf = PathBuf::from(&p);

        if !path.try_exists()? {
            fs::File::create(path.to_owned())?;
        };

        Ok(path)
    } else if let Some(home) = dirs::config_dir() {
        let path: PathBuf = [home.as_path(), Path::new(".boil/config.toml")].iter().collect();

        if !path.try_exists()? {
            fs::File::create(path.to_owned())?;
        };

        Ok(path)
    } else {
        Err(BoilError::ConfigCreate)
    }
}

pub fn default_proj_path() -> PathBuf {
    if let Ok(p) = env::var("BOIL_PROJ_PATH") {
        let path: PathBuf = PathBuf::from(&p);

        if !path.try_exists().unwrap() {
            fs::create_dir_all(path.to_owned()).unwrap();
        };

        path
    } else if let Some(home) = dirs::home_dir() {
        let path: PathBuf = [home.as_path(), Path::new("dev")].iter().collect();

        path
    } else {
        panic!()
    }
}