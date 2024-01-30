use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    programs: Programs
}

#[derive(Serialize, Deserialize)]
pub struct Programs(HashMap<String, Program>);

#[derive(Serialize, Deserialize)]
pub struct Program {
    pub alias: String,
    pub path: PathBuf,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>
}