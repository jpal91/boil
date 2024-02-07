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

    pub fn init(args: InitArgs, path: Option<PathBuf>) -> BoilResult<()> {
        let cfg_path = match path {
            Some(p) => p,
            None => default_config()?
        };

        if !args.force && cfg_path.try_exists()? {
            return Err(BoilError::ConfigExists(cfg_path.to_str().unwrap().to_owned()))
        }


        let res: bool = if !args.test && !args.force {
            user_input(colorize!(b->"Create new boil config at ", bFg->cfg_path.to_str().unwrap(), b->" - [y/N]"))?
        } else {
            true
        };

        if !res {
            return Ok(())
        };

        if let Some(parent) = cfg_path.parent() {
            if !parent.try_exists()? {
                fs::create_dir_all(parent)?;
            };
        } else {
            return Err(BoilError::ConfigCreate)
        };

        fs::File::create(&cfg_path)?;

        let mut config = Config::from(&cfg_path)?;

        if let Some(p) = args.path {
            config.set_proj_path(&p);
        }

        config.write(&cfg_path)?;

        print_color!(Fgb->"Successfully created new boil config");

        Ok(())
    }

    fn add_existing(&mut self, args: AddArgs) -> BoilResult<()> {
        let (description, tags, name, path) = 
            (args.description, args.tags, args.name, args.path);
        
        if self.config.exists(&name) {
            return Err(BoilError::NameExists(name))
        }
        
        let project = match metadata(&path)?.file_type() {
            f if f.is_dir() => true,
            f if f.is_file() => false,
            _ => return Err(BoilError::InvalidPath(path.to_owned()))
        };

        let prog_type = match &args.prog_type {
            Some(p) => ProgType::from_string(p),
            None => ProgType::new()
        };

        let program = Program { name, description, project, prog_type, path, tags };

        let name = program.name.to_owned();
        self.config_mut().insert(program.name.to_owned(), program);

        print_color!(Fgb->"Successfully added", b->&name, Fgb->"to config");

        Ok(())
    }

    fn add_new(&mut self, args: NewArgs) -> BoilResult<()>{
        let program: Program = self.parse_new(&args)?;

        // if program.project {
        //     if let Some(p) = program.path.parent() {
        //         if p.try_exists()? {
        //             return Err(BoilError::PathExists(p.to_path_buf()))
        //         }
        //     } else {
        //         return Err(BoilError::PathExists(program.path))
        //     }
        // }

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
            let name = program.name.to_owned();
            self.config_mut().insert(name.to_owned(), program);
            print_color!(Fgb->"Successfully added", b->&name, Fgb->"to config")
        } else {
            println!("{}", &program.path.to_string_lossy());
            self.config.temp = program
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
                    dir_path.push(p);
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

        print_color!(Fgb->"Successfully updated", b->args.name.as_str());
        Ok(())
    }

    fn list(&self, mut args: ListArgs) -> BoilResult<()> {
        let temp = args.temp;
        if temp {
            println!("{}", self.config.temp.path.to_string_lossy());
            return Ok(())
        }
        let mut table = BoilTable::from_args(args)?;
        table.display(self.config.values());
        
        
        Ok(())
    }

    fn remove(&mut self, mut args: RemoveArgs) -> BoilResult<()> {
        if !args.force {
            let mut input = String::new();
            print!("Are you sure you wish to remove {} - [y/N]: ", &args.name);
            io::stdout().flush();
            io::stdin().read_line(&mut input)?;
            
            if input.as_str().trim() != "y" {
                return Ok(())
            }
        }

        self.config.remove(args.name.to_owned())?;

        print_color!(Fgb->"Successfully removed", b->&args.name, Fgb->"from config");
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

#[cfg(test)]
mod config_tests {
    use self::args::EditOptsGroup;

    use super::*;
    use std::{env, str::FromStr};
    use tempfile::{tempfile, TempDir, tempdir};
    use rstest::*;
    use args::InitArgs;
    

    #[fixture]
    fn temp_dir() -> TempDir {
        let dir = tempdir().unwrap();
        dir
    }

    #[fixture]
    fn config(temp_dir: TempDir) -> TempDir {
        let mut dir_path = PathBuf::from(temp_dir.path().to_owned());
        let mut config = Config::default();
        config.defaults.proj_path = dir_path.to_owned();
        let descriptions = [
            Some(String::from("Fun program")),
            Some(String::from("Utility program")),
            None
        ];
        let tag_list = [
            Some(vec!["Util".to_string(), "fun".to_string(),]),
            Some(vec!["Wonderful".to_string(), "Fun".to_string(),]),
            Some(vec!["util".to_string(), "Other".to_string(),])
        ];

        dir_path.push("null");

        for i in 0..3 {
            let name = format!("test{}", i);
            dir_path.set_file_name(&name);
            let project = false;
            let path = dir_path.to_owned();
            let prog_type = ProgType::Python;
            let description = descriptions[i].to_owned();
            let tags = tag_list[i].to_owned();

            let program = Program {
                name: name.to_owned(),
                project,
                path,
                prog_type,
                description,
                tags
            };

            config.insert(name, program);
            fs::File::create(&dir_path);
        }

        dir_path.set_file_name("config.toml");
        config.write(&dir_path).unwrap();
        temp_dir
    }

    #[rstest]
    fn test_init(temp_dir: TempDir) {
        let path = temp_dir.path().join("config1.toml");
        assert!(!path.exists());
        
        let init_args = InitArgs{ force: true, path: None, test: true };
        Boil::init(init_args, Some(path.to_owned())).unwrap();

        assert!(path.exists()) 
    }
    
    #[rstest]
    #[should_panic]
    fn test_init_panic(temp_dir: TempDir) {
        let path = temp_dir.path().join("config2.toml");
        fs::File::create(&path).unwrap();
        let init_args = InitArgs{ force: false, path: None, test: true };
        Boil::init(init_args, Some(path)).unwrap();
    } 

    #[rstest]
    fn test_add_existing(mut config: TempDir) {
        let mut path = PathBuf::from(config.path());
        path.push("config.toml");
        let mut boil = Boil::from(Some(path.to_owned())).unwrap();

        path.set_file_name("test4");
        fs::File::create(&path);

        let name = String::from("test4");
        let description = Some(String::from("Fun program"));
        let tags = Some(vec!["Other".to_string()]);
        let prog_type = Some("Rust".to_string());
        let path = path.to_owned();

        boil.add_existing(AddArgs{name: name.to_owned(), description: description.to_owned(), tags: tags.to_owned(), prog_type: prog_type.to_owned(), path: path.to_owned()}).unwrap();

        let entry = boil.config.get(String::from("test4")).unwrap();

        assert_eq!(name, entry.name);
        assert_eq!(description, entry.description);
        assert_eq!(tags, entry.tags);
        assert_eq!(ProgType::Rust, entry.prog_type);
        assert_eq!(path, entry.path);
    }

    #[rstest]
    fn test_add_new(config: TempDir) {
        let mut path = PathBuf::from(config.path());
        path.push("config.toml");
        let mut boil = Boil::from(Some(path.to_owned())).unwrap();

        let name = String::from("test4");
        let description = Some(String::from("Fun program"));
        let tags = Some(vec!["Other".to_string()]);
        let prog_type = Some("Bash".to_string());
        let project = true;

        let mut args = NewArgs {
            name: Some(name.to_owned()),
            description: description.to_owned(),
            temp: false,
            project: true,
            prog_type: prog_type.to_owned(),
            tags: tags.to_owned(),
            path: None
        };

        boil.add_new(args.clone()).unwrap();
        path.set_file_name("test4");

        assert!(path.exists());

        let entry = boil.config.get(String::from("test4")).unwrap();

        assert_eq!(name, entry.name);
        assert_eq!(description, entry.description);
        assert_eq!(tags, entry.tags);
        assert_eq!(ProgType::Bash, entry.prog_type);
        assert_eq!(path, entry.path);
        assert_eq!(true, entry.project);

        // Errors
        // Duplicate
        assert!(boil.add_new(args.clone()).is_err());

        // Path already exists
        args.name = Some(String::from("test5"));
        args.path = Some(path.to_owned());
        assert!(boil.add_new(args.clone()).is_err());
        
        // Extension on a project
        path.set_extension("txt");
        args.path = Some(PathBuf::from(path.as_path()));
        assert!(boil.add_new(args.clone()).is_err());

    }

    #[rstest]
    fn test_edit(config: TempDir) {
        let mut path = PathBuf::from(config.path());
        path.push("config.toml");
        let mut boil = Boil::from(Some(path.to_owned())).unwrap();
        let prog = boil.config.get(String::from("test2")).unwrap().to_owned();

        let args = EditArgs {
            name: String::from("test2"),
            eopts: EditOptsGroup {
                description: Some(String::from("Not fun program")),
                tags: Some(vec!["test".to_string()]),
                rm_tags: Some(vec!["util".to_string()]),
                prog_type: None
            }
        };

        boil.edit(args).unwrap();
        let new_prog = boil.config.get(String::from("test2")).unwrap().to_owned();
        assert_ne!(prog.description, new_prog.description);
        assert_ne!(prog.tags, new_prog.tags);
        assert_eq!(prog.prog_type, new_prog.prog_type);
    }

    #[rstest]
    fn test_remove(config: TempDir) {
        let mut path = PathBuf::from(config.path());
        path.push("config.toml");
        let mut boil = Boil::from(Some(path.to_owned())).unwrap();

        let args = RemoveArgs {
            name: String::from("test1"),
            force: true
        };

        boil.remove(args).unwrap();
        let prog = boil.config.get(String::from("test1"));
        println!("{:?}", prog);
        assert!(prog.is_none())
    }
}