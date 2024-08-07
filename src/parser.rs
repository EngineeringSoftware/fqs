use crate::errors::FqError;
use crate::expr::*;
use crate::lexer::Lexer;
use crate::lexer::Token;
use std::rc::Rc;

pub trait Visitor {
    fn visit_query(&self, node: &QueryNode);
    fn end_visit_query(&self, node: &QueryNode);
    fn visit_select(&mut self, node: &SelectNode);
    fn end_visit_select(&mut self, node: &SelectNode);
    fn visit_column(&mut self, node: &ColumnNode);
    fn end_visit_column(&mut self, node: &ColumnNode);
    fn visit_limit(&mut self, node: &LimitNode);
    fn end_visit_limit(&mut self, node: &LimitNode);
}

pub trait Node {
    fn accept(&self, visitor: &mut dyn Visitor);
}

//#[derive(Debug)]
pub struct QueryNode {
    statement: SelectNode,
}

impl QueryNode {
    fn new(statement: SelectNode) -> QueryNode {
        QueryNode { statement }
    }
}

impl Node for QueryNode {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_query(self);
        // visit children
        self.statement.accept(visitor);

        visitor.end_visit_query(self);
    }
}

//#[derive(Debug)]
#[allow(dead_code)]
pub struct SelectNode {
    pub columns: Vec<ColumnNode>,
    file_name: String,
    pub limit: Option<LimitNode>,
    pub xwhere: Option<Rc<dyn Expr>>,
}

impl SelectNode {
    fn new(
        columns: Vec<ColumnNode>,
        file_name: String,
        limit: Option<LimitNode>,
        xwhere: Option<Rc<dyn Expr>>,
    ) -> SelectNode {
        SelectNode {
            columns,
            file_name,
            limit,
            xwhere,
        }
    }

    #[allow(dead_code)]
    pub fn file_name(&self) -> &String {
        &self.file_name
    }
}

impl Node for SelectNode {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_select(self);
        for column in &self.columns {
            column.accept(visitor);
        }
        if let Some(limit) = &self.limit {
            limit.accept(visitor);
        }
        visitor.end_visit_select(self);
    }
}

//#[derive(Debug)]
pub struct ColumnNode {
    // Rc as the expression also goes to the execution plan.
    exp: Rc<dyn Expr>,
}

impl ColumnNode {
    fn new(exp: Rc<dyn Expr>) -> ColumnNode {
        ColumnNode { exp }
    }

    pub fn exp(&self) -> Rc<dyn Expr> {
        Rc::clone(&self.exp)
    }
}

impl Node for ColumnNode {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_column(self);
        visitor.end_visit_column(self);
    }
}

//#[derive(Debug)]
pub struct LimitNode {
    pub num: u32,
}

impl LimitNode {
    fn new(num: u32) -> LimitNode {
        LimitNode { num }
    }
}

impl Node for LimitNode {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_limit(self);
        visitor.end_visit_limit(self);
    }
}

// entry point for parsing
pub fn parse_query(mut lexer: Lexer) -> Result<QueryNode, FqError> {
    match lexer.next() {
        Some(Token::SELECT) => {
            let node = parse_select(&mut lexer)?;
            if lexer.is_empty() {
                Ok(QueryNode::new(node))
            } else {
                Err(FqError::parse("It has extra tokens"))
            }
        }
        _ => Err(FqError::parse("Expected 'select'")),
    }
}

fn parse_select(lexer: &mut Lexer) -> Result<SelectNode, FqError> {
    let columns = parse_columns(lexer)?;

    if !matches!(lexer.next(), Some(Token::FROM)) {
        return Err(FqError::parse("Expecting 'from'"));
    }

    // match file name
    let file_name = match lexer.next() {
        Some(Token::PATH(ref s)) => String::from(s),
        Some(Token::ID(ref s)) => String::from(s),
        _ => return Err(FqError::parse("Expecting path to a file")),
    };

    let xwhere = parse_where(lexer)?;
    let limit = parse_limit(lexer)?;
    Ok(SelectNode::new(columns, file_name, limit, xwhere))
}

fn parse_columns(lexer: &mut Lexer) -> Result<Vec<ColumnNode>, FqError> {
    let mut columns: Vec<ColumnNode> = Vec::new();

    let column = parse_column(lexer)?;
    columns.push(column);

    while matches!(lexer.peek(), Some(Token::COMMA)) {
        lexer.next(); // eat comma
        let column = parse_column(lexer)?;
        columns.push(column);
    }

    Ok(columns)
}

fn parse_column(lexer: &mut Lexer) -> Result<ColumnNode, FqError> {
    let token = lexer.peek();
    match token {
        Some(Token::STAR) => {
            lexer.next();
            return Ok(ColumnNode::new(Rc::new(StarConst::new())));
        }
        _ => Ok(ColumnNode::new(parse_additive_expr(lexer)?)),
    }
}

fn parse_additive_expr(lexer: &mut Lexer) -> Result<Rc<dyn Expr>, FqError> {
    let mut exp: Rc<dyn Expr> = parse_multiplicative_expr(lexer)?;

    loop {
        match lexer.peek() {
            Some(Token::PLUS) => {
                lexer.next();
                let right = parse_multiplicative_expr(lexer)?;
                exp = Rc::new(BinExpr::new(Bop::PLUS, exp, right));
            }
            Some(Token::MINUS) => {
                lexer.next();
                let right = parse_multiplicative_expr(lexer)?;
                exp = Rc::new(BinExpr::new(Bop::MINUS, exp, right));
            }
            _ => break,
        }
    }

    Ok(exp)
}

fn parse_multiplicative_expr(lexer: &mut Lexer) -> Result<Rc<dyn Expr>, FqError> {
    let mut exp: Rc<dyn Expr> = parse_atom(lexer)?;

    loop {
        match lexer.peek() {
            Some(Token::STAR) => {
                lexer.next();
                let right = parse_atom(lexer)?;
                exp = Rc::new(BinExpr::new(Bop::MUL, exp, right));
            }
            Some(Token::DIV) => {
                lexer.next();
                let right = parse_atom(lexer)?;
                exp = Rc::new(BinExpr::new(Bop::DIV, exp, right));
            }
            _ => break,
        }
    }

    Ok(exp)
}

fn parse_atom(lexer: &mut Lexer) -> Result<Rc<dyn Expr>, FqError> {
    let exp: Rc<dyn Expr> = match lexer.next() {
        Some(Token::INT(n)) => Rc::new(IntConst::new(*n)),
        Some(Token::FLOAT(n)) => Rc::new(FloatConst::new(*n)),
        Some(Token::STRING(s)) => Rc::new(StrConst::new(s.to_string())),
        Some(Token::TRUE) => Rc::new(BoolConst::new(true)),
        Some(Token::FALSE) => Rc::new(BoolConst::new(false)),
        Some(Token::INTK) => Rc::new(IntCast::new(parse_cast(lexer)?)),
        Some(Token::FLOATK) => Rc::new(FloatCast::new(parse_cast(lexer)?)),
        Some(Token::BOOLK) => Rc::new(BoolCast::new(parse_cast(lexer)?)),
        Some(Token::STRK) => Rc::new(StrCast::new(parse_cast(lexer)?)),
        Some(Token::ID(s)) => {
            let func = s.clone();
            parse_func_call(lexer, func.as_str())?
        }
        Some(Token::COLUMN(_)) => {
            return Err(FqError::parse("Column reference has to be cast"));
        }
        _ => {
            return Err(FqError::parse("Unsupported expression"));
        }
    };

    Ok(exp)
}

fn parse_func_call(lexer: &mut Lexer, func: &str) -> Result<Rc<dyn Expr>, FqError> {
    if !matches!(lexer.next(), Some(Token::LPAREN)) {
        return Err(FqError::parse("Expecting ( for a function call"));
    }

    let args = parse_additive_expr(lexer)?;

    if !matches!(lexer.next(), Some(Token::RPAREN)) {
        return Err(FqError::parse("Expecting ) for a function call"));
    }

    Ok(Rc::new(FuncCall::new(func, args)))
}

fn parse_where(lexer: &mut Lexer) -> Result<Option<Rc<dyn Expr>>, FqError> {
    match lexer.peek() {
        Some(Token::WHERE) => {
            // eat `where`
            lexer.next();

            // parse left expression
            let left = parse_where_expr(lexer)?;
            // get the operator
            let op;
            match lexer.next() {
                Some(Token::GT) => op = Bop::GT,
                Some(Token::LT) => op = Bop::LT,
                Some(Token::EQ) => op = Bop::EQ,
                Some(Token::LE) => op = Bop::LE,
                Some(Token::GE) => op = Bop::GE,
                Some(Token::NE) => op = Bop::NE,
                _ => {
                    return Err(FqError::parse("Unsuppported operator in where expression"));
                }
            }
            // parse the right expression
            let right = parse_where_expr(lexer)?;

            Ok(Some(Rc::new(BinExpr::new(op, left, right))))
        }
        _ => Ok(None),
    }
}

fn parse_where_expr(lexer: &mut Lexer) -> Result<Rc<dyn Expr>, FqError> {
    match lexer.next() {
        Some(Token::INT(n)) => Ok(Rc::new(IntConst::new(*n))),
        Some(Token::FLOAT(n)) => Ok(Rc::new(FloatConst::new(*n))),
        Some(Token::STRING(s)) => Ok(Rc::new(StrConst::new(s.to_string()))),
        Some(Token::TRUE) => Ok(Rc::new(BoolConst::new(true))),
        Some(Token::FALSE) => Ok(Rc::new(BoolConst::new(false))),
        Some(Token::INTK) => Ok(Rc::new(IntCast::new(parse_cast(lexer)?))),
        Some(Token::FLOATK) => Ok(Rc::new(FloatCast::new(parse_cast(lexer)?))),
        Some(Token::BOOLK) => Ok(Rc::new(BoolCast::new(parse_cast(lexer)?))),
        Some(Token::STRK) => Ok(Rc::new(StrCast::new(parse_cast(lexer)?))),
        Some(Token::COLUMN(_)) => Err(FqError::parse("Column references has to be cast")),
        Some(Token::ID(s)) if SCALAR_FUNCS.contains(&s.as_str()) => {
            let func = s.clone();
            parse_func_call(lexer, func.as_str())
        }
        _ => Err(FqError::parse("Unsupported where expression")),
    }
}

fn parse_cast(lexer: &mut Lexer) -> Result<Rc<dyn Expr>, FqError> {
    if let Some(Token::LPAREN) = lexer.next() {
        let col = parse_column_ref(lexer)?;
        if let Some(Token::RPAREN) = lexer.next() {
            return Ok(col);
        } else {
            return Err(FqError::parse("Missing )"));
        }
    } else {
        return Err(FqError::parse("Missing ("));
    }
}

fn parse_column_ref(lexer: &mut Lexer) -> Result<Rc<dyn Expr>, FqError> {
    match lexer.next() {
        Some(Token::COLUMN(n)) => Ok(Rc::new(ColRef::new(*n))),
        _ => Err(FqError::parse("Needs column reference")),
    }
}

fn parse_limit(lexer: &mut Lexer) -> Result<Option<LimitNode>, FqError> {
    match lexer.peek() {
        Some(Token::LIMIT) => {
            lexer.next();
            if let Some(Token::INT(n)) = lexer.next() {
                if *n > 0 {
                    Ok(Some(LimitNode::new(*n as u32)))
                } else {
                    Err(FqError::parse(
                        "Limit has to be followed by a positive number but was {n}",
                    ))
                }
            } else {
                Err(FqError::parse("Limit should be followed by a number"))
            }
        }
        _ => Ok(None),
    }
}
