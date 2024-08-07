use crate::errors::FqError;
use crate::expr::Expr;
use crate::ops::{Limit, Op, Projection, Scan, Selection};
use crate::parser::{ColumnNode, LimitNode, QueryNode, SelectNode, Visitor};
use std::path::Path;
use std::rc::Rc;

/// Visitor to check for query semantics.
pub struct Checker {
    msg: Option<String>,
}

impl Checker {
    pub fn new() -> Checker {
        Checker { msg: None }
    }

    pub fn result(&self) -> Result<(), FqError> {
        match &self.msg {
            Some(msg) => Err(FqError::semantics(msg.to_string())),
            None => Ok(()),
        }
    }

    fn check_columns(&self, _node: &SelectNode) {
        // nop
    }

    // check if file exists
    fn check_file(&mut self, node: &SelectNode) {
        let path = Path::new(node.file_name());
        if !(path.exists() && path.is_file()) {
            self.msg = Some(format!("File does not exist"));
        }
    }
}

impl Visitor for Checker {
    fn visit_query(&self, _node: &QueryNode) {
        // nop
    }

    fn end_visit_query(&self, _node: &QueryNode) {
        // nop
    }

    fn visit_select(&mut self, node: &SelectNode) {
        self.check_columns(node);
        self.check_file(node);
    }

    fn end_visit_select(&mut self, _node: &SelectNode) {
        // nop
    }

    fn visit_column(&mut self, _node: &ColumnNode) {
        // nop
    }

    fn end_visit_column(&mut self, _node: &ColumnNode) {
        // nop
    }

    fn visit_limit(&mut self, _node: &LimitNode) {}

    fn end_visit_limit(&mut self, _node: &LimitNode) {}
}

pub struct Planner {
    operations: Vec<Box<dyn Op>>,
}

impl Planner {
    pub fn new() -> Planner {
        Planner {
            operations: Vec::new(),
        }
    }

    pub fn operations(&self) -> &Vec<Box<dyn Op>> {
        &self.operations
    }
}

impl Visitor for Planner {
    fn visit_query(&self, _node: &QueryNode) {
        // nop
    }

    fn end_visit_query(&self, _node: &QueryNode) {
        // nop
    }

    fn visit_select(&mut self, node: &SelectNode) {
        let mut operations: Vec<Box<dyn Op>> = Vec::new();

        let op = Box::new(Scan::new(node.file_name().to_string()));
        operations.push(op);

        // selections
        if let Some(xwhere) = &node.xwhere {
            let op = Box::new(Selection::new(Rc::clone(xwhere)));
            operations.push(op);
        }

        // projections
        let mut expressions: Vec<Rc<dyn Expr>> = Vec::new();
        for column in &node.columns {
            expressions.push(column.exp())
        }
        let op = Box::new(Projection::new(expressions));
        operations.push(op);

        // limit as a final selection
        if let Some(limit) = &node.limit {
            let num = limit.num as usize;
            let op = Box::new(Limit::from(Box::new(move |_x, ntable| {
                ntable.nrows() < num
            })));
            operations.push(op);
        }

        self.operations = operations;
    }

    fn end_visit_select(&mut self, _node: &SelectNode) {}

    fn visit_column(&mut self, _node: &ColumnNode) {}

    fn end_visit_column(&mut self, _node: &ColumnNode) {}

    fn visit_limit(&mut self, _node: &LimitNode) {}

    fn end_visit_limit(&mut self, _node: &LimitNode) {}
}
