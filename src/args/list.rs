use clap::Args;

const FORMAT_LH: &'static str = "\
A comma delimited list of fields to show in the resulting list of programs.

Values:
    n | name
    p | path
    P | project
    t | type (program type)
    d | description
    T | tags

Example:
    # Will only show the fields Name, Project, and Tags
    boil list --format=n,project,T
";

const SORT_LH: &'static str = "\
A comma delimited list of (field, (asc|desc)?) to sort the resulting list.

Refer to format arg for more information on field identifiers. 
All items can be followed by (0 or asc) or (1 or desc) for ascending or decending respectively. 
    (0 or asc) is default unless specified.

Examples:
    # Will sort by field name in ascending order
    boil list --sort=n

    # Will sort by if the item is a project (false first in this case), 
    # then tags in ascending order, finally name in ascending order.
    boil list --sort=P,1,t,name,0
";

const FILTER_LH: &'static str = "\
A comma delimited list of (value:expression:field) to filter the resulting list.

Refer to format arg for more information on field identifiers. 
Input must be in the format of value:expression:field

Available Expressions:
    equals | eq <- Field equals value
    notequals | neq | ne <- Field does not equal value
    in <- Value IS contained in field
    nin | notin <- Value IS NOT contained in field

Examples:
    # Will match any items with the name 'my-program'
    boil list --filter=my-program:eq:name

    # Will match any items with the name 'my-program' or items that contain the tag 'fun'
    boil list --filter=my-program:equals:n,fun:in:tags

Note:
    Searches are case-INsensitive, so '--filter=program:eq:n' is the same as '--filter=PROGRAM:eq:n'.
    To do a case-sensitive search, add a '*' to the value (ie '--filter=*program:eq:name)

Tags:
    When using 'in' or 'notin' you can also specify multiple values for tags using a '+'
        (ie '--filter=tiresome+boring:nin:tags).
    This will list results where the item does not have the tags 'tiresome' or 'boring'
";

pub struct SortOpt(pub ListOpts, pub u8);

#[derive(Clone, Debug, PartialEq)]
pub struct FilterOpt(pub ListOpts, pub u8, pub String);

#[derive(Args, Debug, PartialEq, Clone)]
pub struct ListArgs {
    /// A comma delimited list of fields to show in the resulting list of programs
    #[arg(long, value_delimiter=',', require_equals=true, default_value="name,project,type,description,tags", long_help=FORMAT_LH)]
    pub format: Option<Vec<String>>,

    /// A comma delimited list of (field, (asc|desc)?) to sort the resulting list
    #[arg(long, value_delimiter=',', require_equals=true, long_help=SORT_LH)]
    pub sort: Option<Vec<String>>,

    /// A comma delimited list of (value:expression:field) to filter the resulting list
    #[arg(long, value_delimiter=',', require_equals=true, value_parser=parse_filter, long_help=FILTER_LH)]
    pub filter: Option<Vec<FilterOpt>>,

    /// Show the last added temp file
    #[arg(short, long)]
    pub temp: bool
}

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
        return Err(format!("Input must be in format value:expression:field"));
    }

    let val = args.next().unwrap().to_string();

    let exp = match args.next().unwrap() {
        "eq" | "equals" => 0,
        "ne" | "nequals" | "neq" => 1,
        "in" if !val.contains("+") => 2,
        "nin" | "notin" if !val.contains("+") => 3,
        "in" => 4,
        "nin" | "notin" => 5,
        f => return Err(format!("'{}' is not a valid option for 'expression'", f)),
    };

    let field = match args.next().unwrap() {
        "n" | "name" => ListOpts::Name,
        "p" | "path" => ListOpts::Path,
        "P" | "project" => ListOpts::Project,
        "t" | "type" => ListOpts::Type,
        "d" | "description" => ListOpts::Description,
        "T" | "tag" | "tags" => ListOpts::Tags,
        f => return Err(format!("'{}' is not a valid option for 'field'", f)),
    };

    Ok(FilterOpt(field, exp, val))
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::*;
    use crate::args::{Cli, Commands};

    #[test]
    fn test_list() {
        let args = Cli::parse_from([
            "prog",
            "list",
            "--format=n,p,d",
            "--sort=n,1,p",
            "--filter=my-program:eq:name,val2:in:d",
        ]);
        let format = Some(vec!["n".to_string(), "p".to_string(), "d".to_string()]);
        let sort = Some(vec!["n".to_string(), "1".to_string(), "p".to_string()]);
        let filter: Option<Vec<FilterOpt>> = Some(vec![
            FilterOpt(ListOpts::Name, 0, String::from("my-program")),
            FilterOpt(ListOpts::Description, 2, String::from("val2")),
        ]);

        assert_eq!(
            args.command,
            Commands::List(ListArgs {
                format,
                sort,
                filter,
                temp: false
            })
        )
    }

    #[test]
    fn test_parse_filter() {
        assert_eq!(
            Ok(FilterOpt(ListOpts::Project, 1, String::from("false"))),
            parse_filter("false:neq:P")
        );
        assert_eq!(
            Ok(FilterOpt(ListOpts::Tags, 4, String::from("some+else"))),
            parse_filter("some+else:in:T")
        );
        assert_eq!(
            Ok(FilterOpt(ListOpts::Path, 3, String::from("some"))),
            parse_filter("some:nin:p")
        );
        assert_eq!(
            Err(String::from(
                "Input must be in format value:expression:field"
            )),
            parse_filter("too:many:args:in:this")
        );
        assert_eq!(
            Err(format!(
                "'{}' is not a valid option for 'expression'",
                String::from("xeq")
            )),
            parse_filter("some:xeq:n")
        );
        assert_eq!(
            Err(format!(
                "'{}' is not a valid option for 'field'",
                String::from("z")
            )),
            parse_filter("some:eq:z")
        )
    }
}
