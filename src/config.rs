use std::fs;
use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use toml;

use crate::error::BoilResult;
use crate::defaults::default_proj_path;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
    #[serde(default)]
    pub programs: Programs,

    #[serde(default)]
    pub defaults: DefCfg,

    #[serde(default)]
    pub temp: Temp
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DefCfg {
    pub proj_path: PathBuf
}

impl Default for DefCfg {
    fn default() -> Self {
        Self {
            proj_path: default_proj_path()
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Temp {
    pub path: PathBuf
}

pub type ProgMap = HashMap<String, Program>;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Programs(pub HashMap<String, Program>);

#[derive(Serialize, Deserialize, Default, Debug)]
pub enum ProgType {
    Python,
    Rust,
    JavaScript,
    #[default]
    Bash
}

impl ProgType {
    pub fn ext(&self) -> String {
        match self {
            Self::Python => ".py".into(),
            Self::JavaScript => ".js".into(),
            Self::Rust => ".rs".into(),
            _ => ".sh".into()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Program {
    pub name: String,
    pub project: bool,
    pub path: PathBuf,
    #[serde(rename = "type")]
    pub prog_type: ProgType,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>
}

impl Config {
    pub fn from(path: &PathBuf) -> BoilResult<Self> {
        let config: Config;

        if path.try_exists().unwrap_or(false) {
            let content = fs::read_to_string(path)?;
            config = toml::from_str(&content)?;
        } else {
            fs::File::create(path);
            config = Config::default();
        }

        Ok(config)
    }

    pub fn write(&self, path: &PathBuf) -> BoilResult<()> {
        let config_str = toml::to_string_pretty(&self)?;

        fs::write(path, config_str)?;

        Ok(())
    }
}