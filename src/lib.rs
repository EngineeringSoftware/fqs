pub mod args;
mod errors;
mod expr;
mod lexer;
mod ops;
mod parser;
mod table;
mod visitors;

use crate::args::Args;
use crate::errors::FqError;
use crate::lexer::Lexer;
use crate::ops::Engine;
use crate::parser::Node;
use crate::table::Table;
use crate::visitors::{Checker, Planner};

pub fn query(args: Args) -> Result<Table, FqError> {
    let lexer = Lexer::from(&args.query())?;
    let ast = parser::parse_query(lexer)?;

    let mut visitor = Checker::new();
    ast.accept(&mut visitor);
    if let Err(err) = visitor.result() {
        return Err(err);
    }

    let mut planner = Planner::new();
    ast.accept(&mut planner);

    let engine = Engine::new();
    engine.exe(planner.operations())
}
