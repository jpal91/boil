use std::path::PathBuf;

use clap::{Parser, Subcommand, Args};
use prettytable::{Table, Row, Cell, row};

use crate::error::{BoilResult, BoilError};

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
    Edit(EditArgs),

    /// View/List the current programs
    List(ListArgs)
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
pub struct EditArgs {
    #[command(flatten)]
    pub eopts: EditOptsGroup,

    /// Name of entry
    #[arg(requires="EditOptsGroup")]
    pub name: String,
}

#[derive(Args, Debug)]
#[group(multiple=true)]
pub struct EditOptsGroup {
    /// Edit description of entry
    /// 
    /// Ex. -d "My cool app" | --description="My Cool app"
    #[arg(short, long)]
    pub description: Option<String>,

    /// Add list of tags to entry with ',' delimiter
    /// 
    /// Ex. -t one,two,three | --add-tags=one,two,three
    #[arg(short='t', long="add-tags", value_delimiter=',')]
    pub tags: Option<Vec<String>>,

    /// Remove tags from entry
    /// 
    /// Same formatting as 't'. Any tags not found in entry will be
    /// ignored.
    #[arg(short='R', long="rm-tags", value_delimiter=',')]
    pub rm_tags: Option<Vec<String>>,

    /// Edit program type of entry
    #[arg(short, long)]
    pub prog_type: Option<String>,
}

#[derive(Args, Debug)]
pub struct ListArgs {
    #[arg(long, value_delimiter=',', default_value="n,p,P,t,d,T")]
    pub format: Option<Vec<String>>,
    pub name: String
}

pub enum ListOpts {
    Name,
    Path,
    Project,
    Type,
    Description,
    Tags,
}

impl ListArgs {
    pub fn get_opts(&self) -> BoilResult<Vec<ListOpts>> {
        let opts = self.format.as_ref().unwrap();
        let mut header: Vec<ListOpts> = vec![];
        
        for opt in opts.iter() {
            let o = match opt.as_str() {
                "n" | "name" => ListOpts::Name,
                "p" | "path" => ListOpts::Path,
                "P" | "project" => ListOpts::Project,
                "t" | "type" => ListOpts::Type,
                "d" | "description" => ListOpts::Description,
                "T" | "tags" => ListOpts::Tags,
                f => return Err(BoilError::ListFormat(f.to_string()))
            };
            header.push(o);
        }

        Ok(header)
    }
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