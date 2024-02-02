use std::collections::HashSet;

use prettytable::{Cell, Row, Table};

use crate::args::{ListArgs, ListOpts, SortOpt};
use crate::config::Program;
use crate::error::{BoilError, BoilResult};
use crate::utils::capitalize;

struct TableOpts {
    list_args: Vec<ListOpts>,
    sort_arg: Option<Vec<SortOpt>>,
    // filter_args:
}

pub struct BoilTable {
    table: Table,
    opts: TableOpts,
}

type SortKey = Vec<Vec<i8>>;

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
                f => return Err(BoilError::ListFormat(f.to_string())),
            };
            list_args.push(o);
        }

        let sort_arg: Option<Vec<SortOpt>> = if let Some(s) = args.sort {
            let mut opts = s.iter().peekable();
            let mut sort_opts: Vec<SortOpt> = vec![];

            while let Some(opt) = opts.next() {
                let by = match opt.as_str() {
                    "n" | "name" => ListOpts::Name,
                    "p" | "path" => ListOpts::Path,
                    "P" | "project" => ListOpts::Project,
                    "t" | "type" => ListOpts::Type,
                    "d" | "description" => ListOpts::Description,
                    "T" | "tags" => ListOpts::Tags,
                    f => return Err(BoilError::SortFormat(f.to_string())),
                };

                let mut ord: u8 = 0;

                if let Some(&o) = opts.peek() {
                    match o.as_str() {
                        "0" | "asc" => {
                            opts.next();
                        }
                        "1" | "desc" => {
                            opts.next();
                            ord = 1
                        }
                        _ => {}
                    }
                }
                sort_opts.push(SortOpt(by, ord))
            }

            Some(sort_opts)
        } else {
            None
        };

        Ok(Self {
            list_args,
            sort_arg,
        })
    }
}

impl BoilTable {
    pub fn from_args(args: ListArgs) -> BoilResult<Self> {
        let mut new = Self {
            table: Table::new(),
            opts: TableOpts::from_args(args)?,
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
                ListOpts::Type => Cell::new("Type").style_spec("b"),
            };
            first_row.push(o);
        }

        self.table.add_row(Row::new(first_row));
    }

    pub fn display(&mut self, mut entries: Vec<Program>) {
        if let Some(s) = &self.opts.sort_arg {
            entries.sort_by_cached_key(|k| get_sort_key(k, s))
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
                            Cell::new("True").style_spec("bFg")
                        } else {
                            Cell::new("False").style_spec("bFr")
                        }
                    }
                    ListOpts::Tags => {
                        if let Some(t) = &e.tags {
                            Cell::new(
                                t.iter()
                                    .map(|x| capitalize!(x.to_owned()))
                                    .collect::<Vec<String>>()
                                    .join(", ")
                                    .as_str(),
                            )
                        } else {
                            Cell::new("None").style_spec("b")
                        }
                    }
                    ListOpts::Type => Cell::new(&format!("{:?}", e.prog_type)).style_spec("b"),
                };
                row.push(o);
            }
            self.table.add_row(Row::new(row));
        }
        self.table.printstd()
    }
}

fn get_sort_key(prog: &Program, sort_opt: &Vec<SortOpt>) -> SortKey {
    let mut key_order: SortKey = vec![];

    for opt in sort_opt {
        let mut bytes: Vec<u8> = match opt.0 {
            ListOpts::Name => prog.name.as_bytes().into(),
            ListOpts::Path => prog.path.to_str().unwrap().as_bytes().to_vec(),
            ListOpts::Project => vec![prog.project.into()],
            ListOpts::Type => format!("{:?}", prog.prog_type).as_bytes().to_vec(),
            ListOpts::Description => prog
                .description
                .clone()
                .unwrap_or(String::new())
                .as_bytes()
                .to_vec(),
            ListOpts::Tags => prog
                .tags
                .clone()
                .unwrap_or(vec![])
                .iter()
                .flat_map(|f| f.as_bytes().to_owned())
                .collect(),
        };

        let ibytes: Vec<i8> = bytes
            .iter()
            .map(|b| {
                if opt.1 == 0 {
                    *b as i8
                } else {
                    (*b as i8) * -1
                }
            })
            .collect();

        key_order.push(ibytes)
    }

    key_order
}
