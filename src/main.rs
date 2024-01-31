#![allow(unused)]
use std::process::ExitCode;
// use std::io::{self, Write};
use std::env;
use std::path::{PathBuf, Path};

use clap::Parser;
use dirs;
use dotenv::dotenv;

use boil::Boil;
use boil::args::{Cli, Commands};
use boil::error::BoilResult;

fn main() -> ExitCode {
    dotenv().ok();
    let args = Cli::parse();

    if args.debug {
        println!("{:?}", args);
        // set_dev_env_vars();
    }

    let mut boil = match Boil::from(None) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("boil error: {e}");
            return ExitCode::FAILURE
        }
    };

    if let Err(e) = boil.run(args.command){
        eprintln!("boil error: {e}");
        return ExitCode::FAILURE
    }

    if let Err(e) = boil.write() {
        eprintln!("boil error: {e}");
        return ExitCode::FAILURE
    }
    
    ExitCode::SUCCESS
}

// fn set_dev_env_vars() {
//     let cfg_path: PathBuf = [dirs::home_dir().unwrap().as_path(), Path::new("dev/rs-boil/tests/dev/sample-config.toml")].iter().collect();
//     let def_dir: PathBuf = [dirs::home_dir().unwrap().as_path(), Path::new("dev/rs-boil/tests/dev/")].iter().collect();
    
//     env::set_var("BOIL_DEF_CONFIG", cfg_path.as_os_str());
//     env::set_var("BOIL_PROJ_PATH", def_dir.as_os_str());
// }

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
        // set_dev_env_vars();
        let boil = Boil::from(None).unwrap();
        boil.write();

        assert_eq!(boil.cfg_path, [dirs::home_dir().unwrap(), Path::new("dev/rs-boil/tests/dev/sample-config.toml").to_path_buf()].iter().collect::<PathBuf>());
        assert!(boil.cfg_path.exists());
    }

}