use std::path::PathBuf;

use clap::{Parser, Subcommand, Args};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, after_help="")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(long)]
    pub debug: bool
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a new script or project
    New(NewArgs),
    
    /// Add new script or project to boil
    Add(AddArgs),

    /// Edit existing entries
    Edit(EditArgs)
}

#[derive(Args, Debug)]
pub struct AddArgs {

    /// Description of the program/project
    #[arg(short, long)]
    pub description: Option<String>,
    
    /// Related tags to include as descriptors
    #[arg(short, long, value_delimiter=',')]
    pub tags: Option<Vec<String>>,
    
    /// Program or project type - ie python, rust, etc.
    #[arg(short='T', long="type")]
    pub prog_type: Option<String>,
    
    /// Name of the project
    pub name: String,

    /// Absolute or relative path to the program/project
    pub path: PathBuf,
}

#[derive(Args, Debug)]
pub struct NewArgs {
    /// Create a temp file/directory
    #[arg(short, long)]
    pub temp: bool,
    
    /// Create as a project which will create a new directory and boilerplate
    #[arg(short='D', long)]
    pub project: bool,
    
    /// Absolute or relative path to add the file/project
    #[arg(short, long)]
    pub path: Option<PathBuf>,
    
    /// Program or project type - ie python, rust, etc.
    #[arg(short='T', long="type")]
    pub prog_type: Option<String>,
    
    /// Description of the program/project
    #[arg(short, long)]
    pub description: Option<String>,
    
    /// Related tags to include as descriptors
    #[arg(short='G', long, value_delimiter=',')]
    pub tags: Option<Vec<String>>,
    
    /// Name of the project. Not required if creating a temp program
    #[arg(required_unless_present="temp")]
    pub name: Option<String>
}

#[derive(Args, Debug)]
#[group(multiple=true)]
pub struct EditArgs {
    /// Edit description of entry
    #[arg(short, long)]
    pub description: Option<String>,

    /// Add tags to entry
    #[arg(short='T', long="add-tags", value_delimiter=',')]
    pub tags: Option<Vec<String>>,

    /// Remove tags from entry
    #[arg(short='R', long="rm-tags", value_delimiter=',')]
    pub rm_tags: Option<Vec<String>>,

    /// Edit program type of entry
    #[arg(short, long)]
    pub prog_type: Option<String>,

    /// Name of entry
    #[arg(requires="EditArgs")]
    pub name: String,
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

    // #[test]
    // fn test_group() {
    //     let args = Cli::parse_from(["prog", "edit"]);
    //     match args.command {
    //         Commands::Edit(e) => println!("{}", e.)
    //     }
    // }
}