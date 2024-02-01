use prettytable::{Table, Row, Cell, row};

use crate::config::Program;
use crate::args::{ListArgs, ListOpts, SortOpt};
use crate::error::{BoilResult, BoilError};
use crate::utils::capitalize;

struct TableOpts {
    list_args: Vec<ListOpts>,
    sort_arg: Option<SortOpt>,
    // filter_args:
}

pub struct BoilTable {
    table: Table,
    opts: TableOpts
}

impl TableOpts {
    fn from_args(args: ListArgs) -> BoilResult<Self> {
        let format_opts = args.format.as_ref().unwrap();
        let mut list_args: Vec<ListOpts> = vec![];
        
        for opt in format_opts.iter() {
            let o = match opt.as_str() {
                "n" | "name" => ListOpts::Name,
                "p" | "path" => ListOpts::Path,
                "P" | "project" => ListOpts::Project,
                "t" | "type" => ListOpts::Type,
                "d" | "description" => ListOpts::Description,
                "T" | "tags" => ListOpts::Tags,
                f => return Err(BoilError::ListFormat(f.to_string()))
            };
            list_args.push(o);
        }

        let sort_arg: Option<SortOpt> = if let Some(s) = args.sort {
            if s.len() < 2 {
                None
            } else {
                let mut opts = s.iter();
                let by = match opts.next().unwrap().as_str() {
                    "n" | "name" => ListOpts::Name,
                    "p" | "path" => ListOpts::Path,
                    "P" | "project" => ListOpts::Project,
                    "t" | "type" => ListOpts::Type,
                    "d" | "description" => ListOpts::Description,
                    "T" | "tags" => ListOpts::Tags,
                    f => return Err(BoilError::SortFormat(f.to_string()))
                };
                let ord = match opts.next().unwrap().as_str() {
                    "0" | "asc" => 0,
                    "1" | "desc" => 1,
                    f => return Err(BoilError::SortFormat(f.to_string()))
                };
                Some(SortOpt(by, ord))
            }
        } else {
            None
        };
        
        Ok(Self { list_args, sort_arg })
    }
}

impl BoilTable {
    pub fn from_args(args: ListArgs) -> BoilResult<Self> {
        let mut new = Self {
            table: Table::new(),
            opts: TableOpts::from_args(args)?
        };
        new.add_first_row();
        Ok(new)
    }

    fn add_first_row(&mut self) {
        let mut first_row: Vec<Cell> = vec![];
        
        for opt in self.opts.list_args.iter() {
            let o = match opt {
                ListOpts::Name => Cell::new("Name").style_spec("b"),
                ListOpts::Description => Cell::new("Description").style_spec("b"),
                ListOpts::Path => Cell::new("Path").style_spec("b"),
                ListOpts::Project => Cell::new("Project").style_spec("b"),
                ListOpts::Tags => Cell::new("Tags").style_spec("b"),
                ListOpts::Type => Cell::new("Type").style_spec("b")
            };
            first_row.push(o);
        };

        self.table.add_row(Row::new(first_row));
    }

    pub fn display(&mut self, mut entries: Vec<Program>) {
        if let Some(s) = &self.opts.sort_arg {
            match s.0 {
                ListOpts::Description => entries.sort_by_key(|k| k.description.clone().unwrap_or("".to_string())),
                ListOpts::Name => entries.sort_by_key(|k| k.name.to_owned()),
                ListOpts::Path => entries.sort_by_key(|k| k.path.to_str().unwrap().to_string()),
                ListOpts::Project => entries.sort_by_key(|k| k.project),
                ListOpts::Tags => entries.sort_by_key(|k| k.tags.clone().unwrap()),
                ListOpts::Type => entries.sort_by_key(|k| format!("{:?}", k.prog_type))
            }

            if s.1 == 1 {
                entries.reverse()
            }
        };

        for e in entries.iter() {
            let mut row: Vec<Cell> = vec![];
            
            for opt in self.opts.list_args.iter() {
                let o = match opt {
                    ListOpts::Name => Cell::new(&capitalize!(e.name.to_owned())).style_spec("Fbb"),
                    ListOpts::Description => Cell::new(e.description.as_ref().unwrap()),
                    ListOpts::Path => Cell::new(e.path.to_str().unwrap()).style_spec("b"),
                    ListOpts::Project => {
                        if e.project {
                            Cell::new("T").style_spec("bFg")
                        } else {
                            Cell::new("F").style_spec("bFr")
                        }
                    },
                    ListOpts::Tags => {
                        if let Some(t) = &e.tags {
                            Cell::new(
                                t.iter()
                                .map(|x| capitalize!(x.to_owned()))
                                .collect::<Vec<String>>()
                                .join(", ")
                                .as_str()
                            )
                        } else {
                            Cell::new("None").style_spec("b")
                        }
                        
                    },
                    ListOpts::Type => Cell::new(&format!("{:?}", e.prog_type)).style_spec("b")
                };
                row.push(o);
            }
            self.table.add_row(Row::new(row));
        }
        self.table.printstd()
    }
}