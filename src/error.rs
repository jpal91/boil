
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
    NotFound(String)
}

pub type BoilResult<T> = Result<T, BoilError>;