use std::fs;
use std::collections::HashMap;
use std::path::PathBuf;
use std::collections::hash_map::Iter;

use serde::{Deserialize, Serialize};
use toml;
use prettytable::{Table, Row, Cell, row};

use crate::error::BoilResult;
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

    pub fn list(&self, opts: Vec<ListOpts>) {
        let mut table = Table::new();
        let mut first_row: Vec<Cell> = vec![];
        
        for opt in opts.iter() {
            let o = match opt {
                ListOpts::Name => Cell::new("Name").style_spec("b"),
                ListOpts::Description => Cell::new("Description").style_spec("b"),
                ListOpts::Path => Cell::new("Path").style_spec("b"),
                ListOpts::Project => Cell::new("Project").style_spec("b"),
                ListOpts::Tags => Cell::new("Tags").style_spec("b"),
                ListOpts::Type => Cell::new("Type").style_spec("b")
            };
            first_row.push(o);
        };

        table.add_row(Row::new(first_row));
        
        for (_, v) in self.iter() {
            let mut row: Vec<Cell> = vec![];
            
            for opt in opts.iter() {
                let o = match opt {
                    ListOpts::Name => Cell::new(&capitalize!(v.name.to_owned())).style_spec("Fbb"),
                    ListOpts::Description => Cell::new(v.description.as_ref().unwrap()),
                    ListOpts::Path => Cell::new(v.path.to_str().unwrap()).style_spec("b"),
                    ListOpts::Project => {
                        if v.project {
                            Cell::new("T").style_spec("bFg")
                        } else {
                            Cell::new("F").style_spec("bFr")
                        }
                    },
                    ListOpts::Tags => {
                        if let Some(t) = &v.tags {
                            Cell::new(
                                t.iter()
                                .map(|x| capitalize!(x.to_owned()))
                                .collect::<Vec<String>>()
                                .join(", ")
                                .as_str()
                            )
                        } else {
                            Cell::new("None").style_spec("b")
                        }
                        
                    },
                    ListOpts::Type => Cell::new(&format!("{:?}", v.prog_type)).style_spec("b")
                };
                row.push(o);
            }
            table.add_row(Row::new(row));
        };

        table.printstd();
    }
}

macro_rules! capitalize {
    ($string:expr) => {
        if $string.is_empty() {
            $string
        } else {
            let s = $string.to_owned();
            let mut b = s.chars();
            b.next().unwrap().to_uppercase().collect::<String>() + b.as_str()
        }
    };
}
pub(crate) use capitalize;