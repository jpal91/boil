#![allow(unused)]
use std::process::ExitCode;
use std::env;
use std::path::{PathBuf, Path};

use clap::Parser;
use dotenv::dotenv;

use boil::Boil;
use boil::args::{Cli, Commands};
use boil::error::BoilResult;

fn main() -> ExitCode {
    dotenv().ok();
    let args = Cli::parse();

    if args.debug {
        println!("{:?}", args);
    }


    if let Commands::Init(a) = args.command {
        match Boil::init(a, args.cfg_path) {
            Ok(_) => return ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("boil error: {e}");
                return ExitCode::FAILURE
            }
        }
    };


    let mut boil = match Boil::from(args.cfg_path) {
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

// Ideas:
// - Customizable payload for creation from yaml - ie custom gitignore, etc.
// - Logging