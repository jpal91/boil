use std::{iter::Filter, path::PathBuf};

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
    /// A comma delimited list of fields to show in the resulting list of programs
    /// 
    /// Values:
    ///     n | name
    ///     p | path
    ///     P | project
    ///     t | type (program type)
    ///     d | description
    ///     T | tags
    /// 
    /// Example:
    ///     boil list --format=n,project,T
    /// 
    ///     Will only show the fields Name, Project, and Tags
    #[arg(long, value_delimiter=',', require_equals=true, default_value="name,project,type,description,tags")]
    pub format: Option<Vec<String>>,
    
    /// A comma delimited list of fields to sort the resulting list. Use 'help list' or '--help' for further explanation.
    /// 
    /// Refer to format arg for more information on field identifiers. 
    /// All items can be followed by
    /// (0 or asc) or (1 or desc) for ascending or decending respectively. (0 or asc) is default unless specified.
    /// 
    /// Examples:
    /// 
    ///     boil list --sort=n
    /// 
    ///         Will sort by field name in ascending order
    /// 
    ///     boil list --sort=P,1,t,name,0
    /// 
    ///         Will sort by if the item is a project (false first in this case), 
    ///         then tags in ascending order, finally name in ascending order.
    #[arg(long, value_delimiter=',', require_equals=true)]
    pub sort: Option<Vec<String>>,

    #[arg(long, value_delimiter=',', require_equals=true, value_parser=parse_filter)]
    pub filter: Option<Vec<FilterOpt>>

}
pub struct SortOpt(pub ListOpts, pub u8);

#[derive(Clone, Debug)]
pub struct FilterOpt(pub ListOpts, pub u8, pub String);

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum ListOpts {
    Name,
    Path,
    Project,
    Type,
    Description,
    Tags,
}

fn parse_filter(inp: &str) -> Result<FilterOpt, String> {
    let mut args = inp.split(':');

    if args.clone().count() != 3 {
        return Err(format!("Input must be in format field:expression:value"))
    }

    let field = match args.next().unwrap() {
        "n" | "name" => ListOpts::Name,
        "p" | "path" => ListOpts::Path,
        "P" | "project" => ListOpts::Project,
        "t" | "type" => ListOpts::Type,
        "d" | "description" => ListOpts::Description,
        "T" | "tag" | "tags" => ListOpts::Tags,
        f => return Err(format!("'{}' is not a valid option for 'field'", f))
    };

    let exp = match args.next().unwrap() {
        "eq" | "equals" => 0,
        "ne" | "nequals" => 1,
        "in" => 2,
        "nin" | "notin" => 3,
        f => return Err(format!("'{}' is not a valid option for 'expression'", f))
    };

    let val = args.next().unwrap().to_string();

    Ok(FilterOpt(field, exp, val))
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