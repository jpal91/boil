
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BoilError {
    #[error("Error - {0}")]
    IO(#[from] std::io::Error),
    #[error("Unable to deserialize TOML")]
    DeToml(#[from] toml::de::Error),
    #[error("Unable to serialize TOML")]
    SeToml(#[from] toml::ser::Error)
}

pub type BoilResult<T> = Result<T, BoilError>;