use std::collections::HashSet;

use prettytable::{Cell, Row, Table};

use crate::args::{ListArgs, ListOpts, SortOpt, FilterOpt};
use crate::config::{Program, Field};
use crate::error::{BoilError, BoilResult};
use crate::utils::capitalize;

struct TableOpts {
    list_args: Vec<ListOpts>,
    sort_arg: Option<Vec<SortOpt>>,
    filter_args: Option<Vec<FilterOpt>>
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
            filter_args: args.filter
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

        if let Some(f) = &self.opts.filter_args {
            entries = entries
                .into_iter()
                .filter(|p| check_filter(p, f))
                .collect()
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
        let mut bytes = prog.vals_to_bytes(&opt.0);

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

fn check_filter(prog: &Program, filter_opts: &Vec<FilterOpt>) -> bool {
    for f in filter_opts.iter() {
        let mut case_sensitive = false;
        
        let check_val: Vec<u8> = match f.2.as_str() {
            "False" | "false" | "0" => vec![0],
            "True" | "true" | "1" => vec![1],
            f if f.contains('*') => {
                case_sensitive = true;
                f.replace('*', "").as_bytes().into()
            },
            f => f.to_lowercase().as_bytes().into()
        };

        let prog_val: Vec<u8>;

        if case_sensitive {
            prog_val = prog.vals_to_bytes(&f.0);
        } else {
            prog_val = prog.vals_to_bytes(&f.0).into_iter().map(|v| v.to_ascii_lowercase()).collect();
        }

        let res = match f.1 {
            0 => prog_val == check_val,
            1 => prog_val != check_val,
            2 => prog_val.windows(check_val.len()).any(|w| w == &check_val),
            3 => !prog_val.windows(check_val.len()).any(|w| w == &check_val),
            req @ (4 | 5) => {
                let vals = &mut check_val.split(|&num| num == b'+');

                let res = vals
                    .any(|v|
                        prog_val.windows(v.len()).any(|w| w == v)
                    );
                
                if req == 5 {
                    !res
                } else {
                    res
                }
            }
            _ => panic!()
        };

        if !res {
            return false
        }
    }

    true
}
