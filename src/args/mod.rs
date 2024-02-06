mod list;

pub use list::*;

use std::{iter::Filter, path::PathBuf};

use clap::{Args, Parser, Subcommand};
use prettytable::{row, Cell, Row, Table};

use crate::error::{BoilError, BoilResult};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, after_help="")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(long)]
    pub debug: bool,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
    /// Create a new script or project
    New(NewArgs),

    /// Add new script or project to boil
    Add(AddArgs),

    /// Edit existing entries
    Edit(EditArgs),

    /// View/List the current programs
    List(ListArgs),

    /// Remove a program from the configuration
    Remove(RemoveArgs),

    /// Initialize new configuration
    Init(InitArgs),
}

#[derive(Args, Debug, PartialEq)]
pub struct AddArgs {
    /// Description of the program/project
    #[arg(short, long)]
    pub description: Option<String>,

    /// Related tags to include as descriptors
    ///
    /// Example: boil add --tags=fun,util my-program /path/to/program
    #[arg(short, long, value_delimiter = ',')]
    pub tags: Option<Vec<String>>,

    /// Program or project type - ie python, rust, etc.
    #[arg(short = 'T', long = "type")]
    pub prog_type: Option<String>,

    /// Name of the project
    pub name: String,

    /// Absolute or relative path to the program/project
    pub path: PathBuf,
}

#[derive(Args, Debug, PartialEq)]
pub struct NewArgs {
    /// Create a temp file/directory
    #[arg(short, long)]
    pub temp: bool,

    /// Create as a project which will create a new directory and boilerplate
    #[arg(short = 'D', long)]
    pub project: bool,

    /// Absolute or relative path to add the file/project
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    /// Program or project type - ie python, rust, etc.
    #[arg(short = 'T', long = "type")]
    pub prog_type: Option<String>,

    /// Description of the program/project
    #[arg(short, long)]
    pub description: Option<String>,

    /// Related tags to include as descriptors
    ///
    /// Example: boil new --tags=fun,util my-program
    #[arg(short = 'G', long, value_delimiter = ',')]
    pub tags: Option<Vec<String>>,

    /// Name of the project. Not required if creating a temp program
    #[arg(required_unless_present = "temp")]
    pub name: Option<String>,
}

#[derive(Args, Debug, PartialEq)]
pub struct EditArgs {
    #[command(flatten)]
    pub eopts: EditOptsGroup,

    /// Name of entry
    #[arg(requires = "EditOptsGroup")]
    pub name: String,
}

#[derive(Args, Debug, PartialEq)]
#[group(multiple = true)]
pub struct EditOptsGroup {
    /// Edit description of entry
    ///
    /// Ex. -d "My cool app" | --description="My Cool app"
    #[arg(short, long)]
    pub description: Option<String>,

    /// Add list of tags to entry with ',' delimiter
    ///
    /// Ex. -t one,two,three | --add-tags=one,two,three
    #[arg(short = 't', long = "add-tags", value_delimiter = ',')]
    pub tags: Option<Vec<String>>,

    /// Remove tags from entry
    ///
    /// Same formatting as 't'. Any tags not found in entry will be
    /// ignored.
    #[arg(short = 'R', long = "rm-tags", value_delimiter = ',')]
    pub rm_tags: Option<Vec<String>>,

    /// Edit program type of entry
    #[arg(short, long)]
    pub prog_type: Option<String>,
}

#[derive(Args, Debug, PartialEq)]
pub struct RemoveArgs {
    /// Force removal without prompting
    #[arg(long, short)]
    pub force: bool,

    /// Name of entry to remove (will not delete any files, only entry)
    pub name: String,
}

#[derive(Args, Debug, PartialEq)]
pub struct InitArgs {
    /// Force creation/override of config file (USE WITH CAUTION!!)
    #[arg(long, short)]
    pub force: bool,

    /// Specify project directory path
    #[arg(long, short)]
    pub path: Option<PathBuf>,
}

#[cfg(test)]
mod tests {
    use std::{ops::Add, str::FromStr};

    use super::*;

    #[test]
    fn test_add() {
        let args = Cli::parse_from([
            "prog",
            "add",
            "-d",
            "Fun little program",
            "--tags=Fun,Util",
            "test",
            "/tmp/something",
        ]);
        let description = Some(String::from("Fun little program"));
        let tags = Some(vec!["Fun".to_string(), "Util".to_string()]);
        let prog_type: Option<String> = None;
        let name = String::from("test");
        let path = PathBuf::from_str("/tmp/something").unwrap();

        assert_eq!(
            args.command,
            Commands::Add(AddArgs {
                description,
                tags,
                prog_type,
                name,
                path
            })
        )
    }

    fn create_new_args() -> NewArgs {
        let description = None;
        let tags = Some(vec!["Fun".to_string(), "Util".to_string()]);
        let prog_type: Option<String> = Some("Python".to_string());
        let name = Some(String::from("test"));
        let path: Option<PathBuf> = None;
        let temp = false;
        let project = true;

        NewArgs {
            description,
            tags,
            prog_type,
            name,
            temp,
            path,
            project,
        }
    }

    #[test]
    fn test_new() {
        let args = Cli::parse_from([
            "prog", "new", "-D", "-G", "Fun,Util", "-T", "Python", "test",
        ]);
        let new_args = create_new_args();
        assert_eq!(args.command, Commands::New(new_args));
    }

    #[test]
    fn test_new_no_name() {
        let args = Cli::parse_from(["prog", "new", "-D", "-G", "Fun,Util", "-T", "Python", "-t"]);
        let mut new_args = create_new_args();
        new_args.temp = true;
        new_args.name = None;
        assert_eq!(args.command, Commands::New(new_args));
    }

    #[test]
    #[should_panic]
    fn test_new_panic() {
        // Args are not marked as temp yet there is no name
        Cli::try_parse_from(["prog", "new", "-D", "-G", "Fun,Util", "-T", "Python"]).unwrap();
    }

    #[test]
    fn test_edit() {
        let args = Cli::parse_from([
            "prog",
            "edit",
            "-d",
            "Fun little program",
            "--add-tags=Fun,Util",
            "-R",
            "Boring",
            "test",
        ]);
        let description = Some(String::from("Fun little program"));
        let tags = Some(vec!["Fun".to_string(), "Util".to_string()]);
        let rm_tags = Some(vec!["Boring".to_string()]);
        let prog_type: Option<String> = None;
        let name = String::from("test");

        assert_eq!(
            args.command,
            Commands::Edit(EditArgs {
                name,
                eopts: EditOptsGroup {
                    description,
                    tags,
                    rm_tags,
                    prog_type
                }
            })
        )
    }

    #[test]
    #[should_panic]
    fn test_edit_panic() {
        // At least one option must be specified on edit
        Cli::try_parse_from(["prog", "edit", "test"]).unwrap();
    }
}
