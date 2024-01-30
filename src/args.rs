use std::path::PathBuf;

use clap::{Parser, Subcommand, Args};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(long)]
    pub debug: bool
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create new script or project
    New(NewArgs),
    
    /// Add new script or project to boil
    Add(AddArgs)
}

#[derive(Args, Debug)]
pub struct AddArgs {
    #[arg(short, long)]
    pub description: Option<String>,
    #[arg(short, long, value_delimiter=',')]
    pub tags: Option<Vec<String>>,
    pub name: String,
    pub path: PathBuf,
}

#[derive(Args, Debug)]
pub struct NewArgs {
    #[arg(short, long)]
    pub temp: bool,
    #[arg(short='D', long)]
    pub project: bool,
    #[arg(short, long)]
    pub path: Option<PathBuf>,
    #[arg(short='T', long="type")]
    pub prog_type: Option<String>,
    #[arg(short, long)]
    pub description: Option<String>,
    #[arg(short='G', long, value_delimiter=',')]
    pub tags: Option<Vec<String>>,
    #[arg(required_unless_present="temp")]
    pub name: Option<String>
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args() {
        let args = Cli::parse_from(["prog", "new", "-G", "one,two,three", "-t"]);
        println!("{args:?}");
        assert!(true)
    }
}