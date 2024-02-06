#![allow(unused)]
#![allow(dead_code)]
pub mod args;
mod defaults;
mod config;
pub mod error;
mod project;
pub mod table;
pub mod utils;

use std::env::temp_dir;
use std::fs::{self, metadata};
use std::path::{Path, PathBuf};
use std::io::{self, Write};

use dirs;
use serde::{Deserialize, Serialize};

use config::{Config, Program, ProgMap, Temp, ProgType};
use args::{AddArgs, Commands, EditArgs, InitArgs, ListArgs, NewArgs, RemoveArgs};
use error::{BoilResult, BoilError};
use defaults::default_config;
use project::{create_program, create_project};
use table::BoilTable;
use utils::{print_color, colorize, user_input};


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
            Commands::New(c) => self.add_new(c)?,
            Commands::Edit(c) => self.edit(c)?,
            Commands::List(c) => self.list(c)?,
            Commands::Remove(c) => self.remove(c)?,
            _ => {}
        };

        Ok(())
    }

    pub fn init(args: InitArgs) -> BoilResult<()> {
        let cfg_path = default_config()?;

        if !args.force && cfg_path.try_exists()? {
            return Err(BoilError::ConfigExists(cfg_path.to_str().unwrap().to_owned()))
        }

        if let Some(parent) = cfg_path.parent() {
            if !parent.try_exists()? {
                fs::create_dir_all(parent)?;
            };
        } else {
            return Err(BoilError::ConfigCreate)
        };

        let res = user_input(colorize!(b->"Create new boil config at ", bFg->cfg_path.to_str().unwrap(), b->" - [y/N]"))?;

        if !res {
            return Ok(())
        };

        let config = Config::from(&cfg_path)?;
        config.write(&cfg_path)?;

        let mut env_path = cfg_path.clone();
        env_path.set_file_name(".env");

        let mut env = fs::File::create(&env_path)?;
        env.write_all(
            format!(
                "BOIL_DEF_CONFIG={}\nBOIL_PROJ_PATH={}",
                cfg_path.to_string_lossy(),
                config.defaults.proj_path.to_string_lossy()
            ).as_bytes()
        )?;

        print_color!(Fgb->"Successfully created new boil config");

        Ok(())
    }

    fn add_existing(&mut self, args: AddArgs) -> BoilResult<()> {
        let (description, tags, name, path) = 
            (args.description, args.tags, args.name, args.path);
        
        if self.config.exists(&name) {
            return Err(BoilError::NameExists(name))
        }
        
        let project = match metadata(path.to_owned())?.file_type() {
            f if f.is_dir() => true,
            f if f.is_file() => false,
            _ => return Err(BoilError::InvalidPath(path.to_owned()))
        };

        let prog_type = match &args.prog_type {
            Some(p) => ProgType::from_string(p),
            None => ProgType::new()
        };

        let program = Program { name, description, project, prog_type, path, tags };

        self.config_mut().insert(program.name.to_owned(), program);

        Ok(())
    }

    fn add_new(&mut self, args: NewArgs) -> BoilResult<()>{
        let program: Program = self.parse_new(&args)?;

        if !program.path.try_exists()? {
            if program.project {
                create_project(&program.path, &program.prog_type)?;
            } else {
                create_program(&program.path, &program.prog_type)?;
            }
        } else {
            return Err(BoilError::PathExists(program.path))
        };

        if !args.temp {
            self.config_mut().insert(program.name.to_owned(), program);
        } else {
            self.config.temp = Temp{ path: program.path }
        }

        Ok(())
    }

    fn parse_new(&self, args: &NewArgs) -> BoilResult<Program> {
        let name = match &args.name {
            Some(n) => n.to_owned(),
            None => self.get_new_name()
        };

        if self.config.exists(&name) {
            return Err(BoilError::NameExists(name))
        }
        
        let mut path: PathBuf = match (args.temp, args.project, &args.path) {
            (true, true, _) => [temp_dir().as_path(), Path::new(&name)].iter().collect(),
            (true, false, _) => temp_dir(),
            (false, proj, Some(p)) => {
                let mut dir_path: PathBuf;

                if proj && p.extension().is_some() {
                    return Err(BoilError::InvalidPath(p.to_path_buf()))
                }

                if p.is_absolute() {
                    dir_path = p.to_path_buf();
                } else {
                    dir_path = self.config.defaults.proj_path.to_owned();
                    dir_path.push(p.to_path_buf());
                }
                dir_path
            },
            (false, true, None) => {
                let mut dir_path = self.config.defaults.proj_path.to_owned();
                dir_path.push(Path::new(&name));
                dir_path
            },
            (false, false, None) => self.config.defaults.proj_path.to_owned()
        };
        
        let prog_type = match &args.prog_type {
            Some(p) => ProgType::from_string(p),
            None => ProgType::new()
        };
        
        if !args.project {
            path.push(&name);
            path.set_extension(prog_type.ext());
        };

        let description = args.description.to_owned();
        let tags = args.tags.to_owned();

        Ok(Program { name, project: args.project, prog_type, path, description, tags })
    }

    fn edit(&mut self, args: EditArgs) -> BoilResult<()> {
        if !self.config.exists(&args.name) {
            return Err(BoilError::NotFound(args.name))
        }
        
        let entry: &mut Program = self.config.get_mut(&args.name);

        if let Some(d) = args.eopts.description {
            entry.description = Some(d);
        };

        if let Some(t) = args.eopts.tags {
            if let Some(ref mut tags) = entry.tags {
                tags.extend(t);
            } else {
                entry.tags = Some(t);
            }
        };

        if let Some(rm) = args.eopts.rm_tags {
            if let Some(ref mut tags) = entry.tags {
                entry.tags = Some(tags
                    .iter()
                    .filter(|&x| !rm.contains(x))
                    .map(|x| x.to_string())
                    .collect())
            }
        }

        if let Some(p) = args.eopts.prog_type {
            entry.prog_type = ProgType::from_string(&p);
        }
        Ok(())
    }

    fn list(&self, mut args: ListArgs) -> BoilResult<()> {
        let mut table = BoilTable::from_args(args)?;
        table.display(self.config.values());
        Ok(())
    }

    fn remove(&mut self, mut args: RemoveArgs) -> BoilResult<()> {
        if !args.force {
            let mut input = String::new();
            print!("Are you sure you wish to remove {} - [y/N]: ", args.name);
            io::stdout().flush();
            io::stdin().read_line(&mut input)?;
            
            if input.as_str().trim() != "y" {
                return Ok(())
            }
        }

        self.config.remove(args.name)?;
        Ok(())
    }

    fn get_new_name(&self) -> String {
        format!("boil{}", self.config.len())
    }

    fn config_mut(&mut self) -> &mut ProgMap {
        &mut self.config.programs.0
    }

    pub fn write(&self) -> BoilResult<()> {
        self.config.write(&self.cfg_path)?;
        Ok(())
    }

}

