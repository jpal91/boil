#![allow(unused)]
use std::env;
use std::path::{PathBuf, Path};

use clap::Parser;
use dirs;

use boil::Boil;
use boil::args::Cli;


fn main() {
    let args = Cli::parse();

    if args.debug {
        set_dev_env_vars();
    }

    let boil = Boil::from(None).unwrap();

    boil.write();
}

fn set_dev_env_vars() {
    let cfg_path: PathBuf = [dirs::home_dir().unwrap().as_path(), Path::new("dev/rs-boil/tests/dev/sample-config.toml")].iter().collect();
    let def_dir: PathBuf = [dirs::home_dir().unwrap().as_path(), Path::new("dev/rs-boil/tests/dev/")].iter().collect();
    
    env::set_var("BOIL_DEF_CONFIG", cfg_path.as_os_str());
    env::set_var("BOIL_PROJ_PATH", def_dir.as_os_str());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{PathBuf, Path};

    #[test]
    fn test_args() {
        let args = Cli::parse_from(["test", "--debug", "new", "something"]);
        println!("{:?}", args);

        assert!(args.debug);
    }

    #[test]
    fn test_create_config() {
        set_dev_env_vars();
        let boil = Boil::from(None).unwrap();
        boil.write();

        assert_eq!(boil.cfg_path, [dirs::home_dir().unwrap(), Path::new("dev/rs-boil/tests/dev/sample-config.toml").to_path_buf()].iter().collect::<PathBuf>());
        assert!(boil.cfg_path.exists());
    }
}