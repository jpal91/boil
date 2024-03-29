
use std::path::PathBuf;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum BoilError {
    #[error("Error - {0}")]
    IO(#[from] std::io::Error),
    #[error("Unable to deserialize TOML")]
    DeToml(#[from] toml::de::Error),
    #[error("Unable to serialize TOML")]
    SeToml(#[from] toml::ser::Error),
    #[error("Unable to create config file")]
    ConfigCreate,
    #[error("Path already exists - {0}")]
    PathExists(PathBuf),
    #[error("{0} - Path is not valid to add as a program")]
    InvalidPath(PathBuf),
    #[error("Program with name '{0}' already exists")]
    NameExists(String),
    #[error("No entry found for '{0}' in config")]
    NotFound(String),
    #[error("Unknown format option - '{0}'")]
    ListFormat(String),
    #[error("Unknown sort option - '{0}'")]
    SortFormat(String),
    #[error("Config file at - '{0}' - already exists")]
    ConfigExists(String),
    #[error("Config file not found - please use 'boil init' to create")]
    NeedInit,
}

pub type BoilResult<T> = Result<T, BoilError>;