#![allow(unused)]
use boil::Boil;
use boil::args::Cli;
use clap::Parser;

fn main() {
    let args = Cli::parse();

    let boil = Boil::from(None).unwrap();

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
        let args = Cli::parse_from(["test", "--debug", "new", "something"]);
        let boil = Boil::_from_debug().unwrap();
        boil.write();

        assert_eq!(boil.cfg_path, [dirs::home_dir().unwrap(), Path::new("dev/rs-boil/tests/sample-config.toml").to_path_buf()].iter().collect::<PathBuf>());
        assert!(boil.cfg_path.exists());
    }
}