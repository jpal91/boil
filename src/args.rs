

use clap::{Parser, Subcommand, Args};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands
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
    name: String
}

#[derive(Args, Debug)]
pub struct NewArgs {
    temp: bool,
    directory: bool,
    prog_type: Option<String>,
    name: Option<String>
}