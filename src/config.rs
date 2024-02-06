use std::fs;
use std::collections::HashMap;
use std::path::PathBuf;
use std::collections::hash_map::Iter;

use serde::{Deserialize, Serialize};
use toml;
use prettytable::{Table, Row, Cell, row};

use crate::error::{BoilError, BoilResult};
use crate::defaults::default_proj_path;
use crate::args::ListOpts;

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

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Temp {
    pub path: PathBuf
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Programs(pub ProgMap);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Program {
    pub name: String,
    pub project: bool,
    pub path: PathBuf,
    #[serde(rename = "type")]
    pub prog_type: ProgType,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub enum ProgType {
    Python,
    Rust,
    JavaScript,
    #[default]
    Bash
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Field {
    Name(String),
    Project(bool),
    Path(PathBuf),
    Type(ProgType),
    Description(Option<String>),
    Tags(Option<Vec<String>>)
}

pub type ProgMap = HashMap<String, Program>;

impl Default for DefCfg {
    fn default() -> Self {
        Self {
            proj_path: default_proj_path()
        }
    }
}

impl Program {
    pub fn opt_to_val(&self, opt: &ListOpts) -> Field {
        match opt {
            ListOpts::Name => Field::Name(self.name.clone()),
            ListOpts::Description => Field::Description(self.description.clone()),
            ListOpts::Path => Field::Path(self.path.clone()),
            ListOpts::Project => Field::Project(self.project),
            ListOpts::Tags => Field::Tags(self.tags.clone()),
            ListOpts::Type => Field::Type(self.prog_type.clone())
        }
    }

    pub fn vals_to_bytes(&self, opt: &ListOpts) -> Vec<u8> {
        match opt {
            ListOpts::Name => self.name.as_bytes().into(),
            ListOpts::Path => self.path.to_str().unwrap().as_bytes().to_vec(),
            ListOpts::Project => vec![self.project.into()],
            ListOpts::Type => format!("{:?}", self.prog_type).as_bytes().to_vec(),
            ListOpts::Description => self
                .description
                .clone()
                .unwrap_or(String::new())
                .as_bytes()
                .to_vec(),
            ListOpts::Tags => self
                .tags
                .clone()
                .unwrap_or(vec![])
                .iter()
                .flat_map(|f| f.as_bytes().to_owned())
                .collect(),
        }
    }
}

impl ProgType {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn from_string(prog_type: &str) -> Self {
        match prog_type.to_lowercase().as_str() {
            "py" | "python" => ProgType::Python,
            "js" | "javascript" => ProgType::JavaScript,
            "rs" | "rust" => ProgType::Rust,
            _ => ProgType::Bash
        }
    }
    
    pub fn ext(&self) -> String {
        match self {
            Self::Python => "py".into(),
            Self::JavaScript => "js".into(),
            Self::Rust => "rs".into(),
            _ => "sh".into()
        }
    }
}

impl Config {
    pub fn from(path: &PathBuf) -> BoilResult<Self> {
        let config: Config;

        if path.try_exists().unwrap_or(false) {
            let content = fs::read_to_string(path)?;
            config = toml::from_str(&content)?;
        } else {
            return Err(BoilError::NeedInit)
        }

        Ok(config)
    }

    pub fn write(&self, path: &PathBuf) -> BoilResult<()> {
        let config_str = toml::to_string_pretty(&self)?;

        fs::write(path, config_str)?;

        Ok(())
    }

    pub fn remove(&mut self, entry: String) -> BoilResult<()> {
        if self.programs.0.remove_entry(&entry).is_none() {
            Err(BoilError::NotFound(entry))
        } else {
            Ok(())
        }
    }

    pub fn len(&self) -> usize {
        self.programs.0.len()
    }

    pub fn exists(&self, key: &str) -> bool {
        self.programs.0.get(key).is_some()
    }

    pub fn get_mut(&mut self, key: &str) -> &mut Program {
        self.programs.0.get_mut(key).unwrap()
    }

    pub fn iter(&self) -> Iter<String, Program> {
        self.programs.0.iter()
    }

    pub fn values(&self) -> Vec<Program> {
        self.programs.0.values().cloned().collect()
    }

    pub fn set_proj_path(&mut self, val: &PathBuf) {
        self.defaults.proj_path = val.to_owned();
    }

}