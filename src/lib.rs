#![allow(unused)]
#![allow(dead_code)]
pub mod args;
mod config;
mod error;

use std::fs;
use std::path::{Path, PathBuf};

use dirs;

use config::{Config, Program};
use args::{Commands, NewArgs};
use error::{BoilResult, BoilError};

#[derive(Debug, Default)]
pub struct Boil {
    config: Config,
    path: PathBuf,
}

impl Boil {
    pub fn new() -> Self {
        Boil::default()
    }

    pub fn from(p: Option<PathBuf>) -> BoilResult<Self> {
        let path = match p {
            Some(pb) => pb,
            None => default_dir()?,
        };

        let config = Config::from(&path)?;

        Ok(Self { config, path })
    }

    pub fn add(&self, cmd: Commands) {
        match cmd {
            Commands::Add(c) => todo!(),
            Commands::New(c) => todo!()
        }
    }

    fn add_new(&self, args: NewArgs) {
        
    }
}

fn default_dir() -> BoilResult<PathBuf> {
    if let Some(home) = dirs::config_dir() {
        let path: PathBuf = [home.as_path(), Path::new("/.boil/config.toml")].iter().collect();

        if !path.exists() {
            fs::File::create(path.to_owned())?;
        };

        Ok(path)
    } else {
        Err(BoilError::ConfigCreate)
    }
}
