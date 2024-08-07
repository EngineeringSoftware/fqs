use crate::errors::FqError;
use crate::expr::{ColRef, Expr, Val};
use crate::table::Table;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

pub trait Op {
    fn exe(&self, table: Table) -> Result<Table, FqError>;
}

#[derive(Debug)]
pub struct Scan {
    file_name: String,
}

impl Scan {
    pub fn new(file_name: String) -> Scan {
        Scan { file_name }
    }
}

impl Op for Scan {
    fn exe(&self, table: Table) -> Result<Table, FqError> {
        if !table.empty() {
            return Err(FqError::exe("Given table has to be empty"));
        }

        let file = match File::open(&self.file_name) {
            Ok(file) => file,
            Err(_) => {
                return Err(FqError::exe("Failed to open file"));
            }
        };

        let reader = BufReader::new(file);

        let mut content: Vec<Vec<String>> = Vec::new();

        for line in reader.lines() {
            match line {
                Ok(line) => {
                    let split_line: Vec<String> = line.split(" ").map(String::from).collect();
                    // The line below does not match the split done by
                    // the `cut` command.
                    //line.split_whitespace().map(String::from).collect();
                    content.push(split_line);
                }
                Err(_) => {
                    return Err(FqError::exe("An issue reading a line"));
                }
            }
        }

        Ok(Table::from(content))
    }
}

pub struct Selection {
    exp: Rc<dyn Expr>,
}

impl Selection {
    pub fn new(exp: Rc<dyn Expr>) -> Selection {
        Selection { exp }
    }
}

impl Op for Selection {
    fn exe(&self, table: Table) -> Result<Table, FqError> {
        let mut ntable = Table::new();
        let mut iterator = table.iter();
        while let Some(row) = iterator.next() {
            if let Val::BOOL(val) = self.exp.eval(&row, &None)? {
                if val {
                    ntable.push_row(row)?;
                }
            }
        }
        Ok(ntable)
    }
}

//#[derive(Debug)]
pub struct Limit {
    func: Box<dyn Fn(&Vec<String>, &Table) -> bool>,
}

impl Limit {
    pub fn from(func: Box<dyn Fn(&Vec<String>, &Table) -> bool>) -> Limit {
        Limit { func }
    }
}

impl Op for Limit {
    fn exe(&self, table: Table) -> Result<Table, FqError> {
        let mut ntable = Table::new();
        let mut iterator = table.iter();
        while let Some(row) = iterator.next() {
            if (self.func)(&row, &ntable) {
                ntable.push_row(row)?;
            }
        }
        Ok(ntable)
    }
}

//#[derive(Debug)]
pub struct Projection {
    expressions: Vec<Rc<dyn Expr>>,
}

impl Projection {
    pub fn new(expressions: Vec<Rc<dyn Expr>>) -> Projection {
        Projection { expressions }
    }
}

impl Op for Projection {
    fn exe(&self, table: Table) -> Result<Table, FqError> {
        let mut ntable = Table::new();

        // if empty table, then no work to be done here.
        if table.empty() {
            return Ok(ntable);
        }

        // Expand *.
        let mut expressions: Vec<Rc<dyn Expr>> = Vec::new();
        for exp in &self.expressions {
            if exp.is_star() {
                for ix in 0..=table.ncols() - 1 {
                    expressions.push(Rc::new(ColRef::new(ix.try_into().unwrap())));
                }
            } else {
                expressions.push(Rc::clone(&exp));
            }
        }

        for exp in expressions {
            // Accumulator to support aggragate functions.
            let mut acc: Option<Box<Val>> = None;
            // Process one row at a time and save a value in the
            // current column.
            let mut col: Vec<String> = Vec::new();
            let mut iterator = table.iter();
            while let Some(row) = iterator.next() {
                // Note that acc given to eval is never ACC.
                let val = exp.eval(&row, &acc)?;
                // Result from aggragate function is always ACC.
                match val {
                    Val::ACC(val) => acc = Some(val),
                    _ => col.push(val.to_string()),
                }
            }
            if let Some(val) = acc {
                let val = exp.finish(*val, table.nrows() as i32)?;
                col.push(val.to_string());
            }
            ntable.push_col(col)?;
        }

        Ok(ntable)
    }
}

pub struct Engine;

impl Engine {
    pub fn new() -> Engine {
        Engine {}
    }

    pub fn exe(&self, operations: &Vec<Box<dyn Op>>) -> Result<Table, FqError> {
        let mut table = Table::from(vec![]);

        for op in operations {
            table = op.exe(table)?;
        }
        Ok(table)
    }
}
