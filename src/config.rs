use std::fs;
use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use toml;

use crate::error::BoilResult;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    programs: Programs
}

#[derive(Serialize, Deserialize, Default)]
pub struct Programs(HashMap<String, Program>);

#[derive(Serialize, Deserialize)]
pub struct Program {
    pub alias: String,
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