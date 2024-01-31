#![allow(unused)]
#![allow(dead_code)]
pub mod args;
mod defaults;
mod config;
mod error;

use std::env::temp_dir;
use std::fs::{self, metadata};
use std::path::{Path, PathBuf};

use dirs;
use serde::{Deserialize, Serialize};

use config::{Config, Program, ProgMap, Temp};
use args::{Commands, NewArgs, AddArgs};
use error::{BoilResult, BoilError};
use defaults::default_config;


#[derive(Debug, Default)]
pub struct Boil {
    pub config: Config,
    pub cfg_path: PathBuf,
}

impl Boil {
    pub fn new() -> Self {
        Boil::default()
    }

    pub fn from(p: Option<PathBuf>) -> BoilResult<Self> {
        let cfg_path = match p {
            Some(pb) => pb,
            None => default_config()?,
        };

        let config = Config::from(&cfg_path)?;

        Ok(Self { config, cfg_path })
    }

    pub fn run(&mut self, cmd: Commands) -> BoilResult<()> {
        match cmd {
            Commands::Add(c) => self.add_existing(c)?,
            Commands::New(c) => self.add_new(c)?
        };

        Ok(())
    }

    fn add_existing(&mut self, args: AddArgs) -> BoilResult<()> {
        let (description, tags, name, path) = 
            (args.description, args.tags, args.name, args.path);
        
        let project = match metadata(path.to_owned())?.file_type() {
            f if f.is_dir() => true,
            f if f.is_file() => false,
            _ => return Err(BoilError::InvalidPath(path.to_owned()))
        };

        let program = Program { name, description, project, path, tags };

        self.config_mut().insert(program.name.to_owned(), program);

        Ok(())
    }

    fn add_new(&mut self, args: NewArgs) -> BoilResult<()>{
        let program = self.parse_new(&args);

        if !program.path.try_exists()? {
            fs::File::create(&program.path)?;
        } else {
            return Err(BoilError::PathExists)
        };

        if !args.temp {
            self.config_mut().insert(program.name.to_owned(), program);
        } else {
            self.config.temp = Temp{ path: program.path }
        }

        Ok(())
    }

    fn parse_new(&self, args: &NewArgs) -> Program {
        let name = match &args.name {
            Some(n) => n.to_owned(),
            None => self.get_new_name()
        };
        
        let mut path: PathBuf = match (args.temp, args.project, &args.path) {
            (true, true, _) => [temp_dir().as_path(), Path::new(&name)].iter().collect(),
            (true, false, _) => temp_dir(),
            (false, true, p) => {
                let mut dir_path = self.config.defaults.proj_path.to_owned();
                dir_path.push(Path::new(&name));
                dir_path
            },
            (_, _, _) => self.config.defaults.proj_path.to_owned()
        };

        if !args.project {
            let ext = get_file_ext(&args.prog_type);
            path.push(ext)
        };

        let description = args.description.to_owned();
        let tags = args.tags.to_owned();

        Program { name, project: args.project, path, description, tags }
    }

    fn get_new_name(&self) -> String {
        format!("boil{}", self.config.programs.0.len())
    }

    fn config_mut(&mut self) -> &mut ProgMap {
        &mut self.config.programs.0
    }

    pub fn write(&self) -> BoilResult<()> {
        self.config.write(&self.cfg_path)?;
        Ok(())
    }

}

fn get_file_ext(prog_type: &Option<String>) -> String {
    if let Some(p) = prog_type {
        match p.to_lowercase().as_str() {
            "py" | "python" => ".py".into(),
            "js" | "javascript" => ".js".into(),
            "rs" | "rust" => ".rs".into(),
            _ => ".sh".into()
        }
    } else {
        String::from(".sh")
    }

}
