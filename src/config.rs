use std::fs;
use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use toml;

use crate::error::BoilResult;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
    #[serde(default)]
    programs: Programs,

    #[serde(default)]
    defaults: DefCfg
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct DefCfg {
    proj_path: String
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Programs(HashMap<String, Program>);

#[derive(Serialize, Deserialize, Debug)]
pub struct Program {
    pub name: String,
    pub project: bool,
    pub path: PathBuf,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>
}

impl Config {
    pub fn from(path: &PathBuf) -> BoilResult<Self> {
        let content = fs::read_to_string(path)?;

        let config = toml::from_str(&content)?;

        Ok(config)
    }

    pub fn write(&self, path: &PathBuf) -> BoilResult<()> {
        let config_str = toml::to_string_pretty(&self)?;

        fs::write(path, config_str)?;

        Ok(())
    }
}