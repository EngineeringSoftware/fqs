use crate::errors::FqError;
use std::fmt;
use std::rc::Rc;

pub enum Val {
    INT(i32),
    FLOAT(f32),
    STR(String),
    BOOL(bool),
    ACC(Box<Val>),
    NULL,
}

// string functions
const UPPER_FUNC: &'static str = "upper";
const LOWER_FUNC: &'static str = "lower";
const LENGTH_FUNC: &'static str = "length";
const REV_FUNC: &'static str = "rev";
// math functions
const ABS_FUNC: &'static str = "abs";
const SIGN_FUNC: &'static str = "sign";
const CEIL_FUNC: &'static str = "ceil";
const FLOOR_FUNC: &'static str = "floor";
const ROUND_FUNC: &'static str = "round";
const COS_FUNC: &'static str = "cos";
const SIN_FUNC: &'static str = "sin";
// aggragate functions
const SUM_FUNC: &'static str = "sum";
const COUNT_FUNC: &'static str = "count";
const MAX_FUNC: &'static str = "max";
const MIN_FUNC: &'static str = "min";
const AVG_FUNC: &'static str = "avg";

pub static SCALAR_FUNCS: [&str; 11] = [
    UPPER_FUNC,
    LOWER_FUNC,
    LENGTH_FUNC,
    REV_FUNC,
    ABS_FUNC,
    SIGN_FUNC,
    CEIL_FUNC,
    FLOOR_FUNC,
    ROUND_FUNC,
    COS_FUNC,
    SIN_FUNC,
];

pub trait Sign {
    fn sign(&self) -> i32;
}

impl Sign for i32 {
    fn sign(&self) -> i32 {
        match self {
            val if *val > 0 => 1,
            val if *val < 0 => -1,
            _ => 0,
        }
    }
}

impl Sign for f32 {
    fn sign(&self) -> i32 {
        match self {
            val if *val > 0.0 => 1,
            val if *val < 0.0 => -1,
            _ => 0,
        }
    }
}

fn max<T: PartialOrd>(first: T, second: T) -> T {
    if first < second {
        second
    } else {
        first
    }
}

fn min<T: PartialOrd>(first: T, second: T) -> T {
    if first < second {
        first
    } else {
        second
    }
}

// todo: PartialOrd and PartialEq
impl Val {
    // scalar functions

    fn abs(&self) -> Result<Val, FqError> {
        match self {
            Val::INT(val) => Ok(Val::INT(val.abs())),
            Val::FLOAT(val) => Ok(Val::FLOAT(val.abs())),
            Val::NULL => Ok(Val::NULL),
            _ => Err(FqError::exe("abs() only works for int and float types")),
        }
    }

    fn upper(&self) -> Result<Val, FqError> {
        match self {
            Val::STR(val) => Ok(Val::STR(val.to_uppercase())),
            Val::NULL => Ok(Val::NULL),
            _ => Err(FqError::exe("upper() only works for string types")),
        }
    }

    fn lower(&self) -> Result<Val, FqError> {
        match self {
            Val::STR(val) => Ok(Val::STR(val.to_lowercase())),
            Val::NULL => Ok(Val::NULL),
            _ => Err(FqError::exe("lower() only works for string types")),
        }
    }

    fn length(&self) -> Result<Val, FqError> {
        match self {
            Val::STR(val) => Ok(Val::INT(val.bytes().len() as i32)),
            Val::NULL => Ok(Val::NULL),
            _ => Err(FqError::exe("length() only works for string types")),
        }
    }

    fn rev(&self) -> Result<Val, FqError> {
        match self {
            Val::STR(val) => Ok(Val::STR(val.chars().rev().collect())),
            Val::NULL => Ok(Val::NULL),
            _ => Err(FqError::exe("rev() only works for string types")),
        }
    }

    fn sign(&self) -> Result<Val, FqError> {
        match self {
            Val::INT(val) => Ok(Val::INT(val.sign())),
            Val::FLOAT(val) => Ok(Val::INT(val.sign())),
            Val::NULL => Ok(Val::NULL),
            _ => Err(FqError::exe("sign() only works for number types")),
        }
    }

    fn ceil(&self) -> Result<Val, FqError> {
        match self {
            Val::INT(val) => Ok(Val::INT(*val)),
            Val::FLOAT(val) => Ok(Val::FLOAT(val.ceil())),
            Val::NULL => Ok(Val::NULL),
            _ => Err(FqError::exe("ceil() only works for number types")),
        }
    }

    fn floor(&self) -> Result<Val, FqError> {
        match self {
            Val::INT(val) => Ok(Val::INT(*val)),
            Val::FLOAT(val) => Ok(Val::FLOAT(val.floor())),
            Val::NULL => Ok(Val::NULL),
            _ => Err(FqError::exe("floor() only works for number types")),
        }
    }

    fn round(&self) -> Result<Val, FqError> {
        match self {
            Val::INT(val) => Ok(Val::INT(*val)),
            Val::FLOAT(val) => Ok(Val::FLOAT(val.round())),
            Val::NULL => Ok(Val::NULL),
            _ => Err(FqError::exe("round() only works for number types")),
        }
    }

    fn cos(&self) -> Result<Val, FqError> {
        match self {
            Val::INT(val) => Ok(Val::FLOAT((*val as f32).cos())),
            Val::FLOAT(val) => Ok(Val::FLOAT(val.cos())),
            Val::NULL => Ok(Val::NULL),
            _ => Err(FqError::exe("cos() only works for number types")),
        }
    }

    fn sin(&self) -> Result<Val, FqError> {
        let val = match self {
            Val::INT(val) => (*val as f32).sin(),
            Val::FLOAT(val) => val.sin(),
            Val::NULL => {
                return Ok(Val::NULL);
            }
            _ => {
                return Err(FqError::exe("sin() only works for number types"));
            }
        };
        Ok(Val::FLOAT(val))
    }

    // aggragate functions

    fn sum(&self, acc: &Option<Box<Val>>) -> Result<Val, FqError> {
        match self {
            Val::INT(val) => match acc {
                None => Ok(Val::ACC(Box::new(Val::INT(*val)))),
                Some(acc) => match acc.as_ref() {
                    Val::INT(other) => Ok(Val::ACC(Box::new(Val::INT(*val + other)))),
                    Val::FLOAT(other) => Ok(Val::ACC(Box::new(Val::FLOAT((*val as f32) + other)))),
                    _ => Err(FqError::exe("sum() only works for number types")),
                },
            },
            Val::FLOAT(val) => match acc {
                None => Ok(Val::ACC(Box::new(Val::FLOAT(*val)))),
                Some(acc) => match acc.as_ref() {
                    Val::INT(other) => Ok(Val::ACC(Box::new(Val::FLOAT(*val + (*other as f32))))),
                    Val::FLOAT(other) => Ok(Val::ACC(Box::new(Val::FLOAT(*val + other)))),
                    _ => Err(FqError::exe("sum() only works for number types")),
                },
            },
            // value to be processed is null
            Val::NULL => match acc {
                None => Ok(Val::ACC(Box::new(Val::NULL))),
                Some(acc) => Ok(Val::ACC(Box::new(*acc.clone()))),
            },
            _ => Err(FqError::exe("sum() only works for number types")),
        }
    }

    fn count(&self, acc: &Option<Box<Val>>) -> Result<Val, FqError> {
        // Do not count `null` values.
        if let Val::NULL = self {
            match acc {
                None => {
                    return Ok(Val::ACC(Box::new(Val::NULL)));
                }
                Some(acc) => {
                    return Ok(Val::ACC(Box::new(*acc.clone())));
                }
            }
        }
        match acc {
            None => Ok(Val::ACC(Box::new(Val::INT(1)))),
            Some(acc) => match acc.as_ref() {
                Val::INT(val) => Ok(Val::ACC(Box::new(Val::INT(*val + 1)))),
                _ => Err(FqError::exe("Internal error")),
            },
        }
    }

    fn max(&self, acc: &Option<Box<Val>>) -> Result<Val, FqError> {
        match self {
            Val::INT(val) => match acc {
                None => Ok(Val::ACC(Box::new(Val::INT(*val)))),
                Some(acc) => match acc.as_ref() {
                    Val::INT(other) => Ok(Val::ACC(Box::new(Val::INT(max(*val, *other))))),
                    Val::FLOAT(other) => {
                        Ok(Val::ACC(Box::new(Val::FLOAT(max(*val as f32, *other)))))
                    }
                    _ => Err(FqError::exe("max() only works for number types")),
                },
            },
            Val::FLOAT(val) => match acc {
                None => Ok(Val::ACC(Box::new(Val::FLOAT(*val)))),
                Some(acc) => match acc.as_ref() {
                    Val::INT(other) => Ok(Val::ACC(Box::new(Val::FLOAT(max(*val, *other as f32))))),
                    Val::FLOAT(other) => Ok(Val::ACC(Box::new(Val::FLOAT(max(*val, *other))))),
                    _ => Err(FqError::exe("max() only works for number types")),
                },
            },
            Val::NULL => match acc {
                None => Ok(Val::ACC(Box::new(Val::NULL))),
                Some(acc) => Ok(Val::ACC(Box::new(*acc.clone()))),
            },
            _ => Err(FqError::exe("max() only works for number types")),
        }
    }

    fn min(&self, acc: &Option<Box<Val>>) -> Result<Val, FqError> {
        match self {
            Val::INT(val) => match acc {
                None => Ok(Val::ACC(Box::new(Val::INT(*val)))),
                Some(acc) => match acc.as_ref() {
                    Val::INT(other) => Ok(Val::ACC(Box::new(Val::INT(min(*val, *other))))),
                    Val::FLOAT(other) => {
                        Ok(Val::ACC(Box::new(Val::FLOAT(min(*val as f32, *other)))))
                    }
                    _ => Err(FqError::exe("min() only works for number types")),
                },
            },
            Val::FLOAT(val) => match acc {
                None => Ok(Val::ACC(Box::new(Val::FLOAT(*val)))),
                Some(acc) => match acc.as_ref() {
                    Val::INT(other) => Ok(Val::ACC(Box::new(Val::FLOAT(min(*val, *other as f32))))),
                    Val::FLOAT(other) => Ok(Val::ACC(Box::new(Val::FLOAT(min(*val, *other))))),
                    _ => Err(FqError::exe("min() only works for number types")),
                },
            },
            Val::NULL => match acc {
                None => Ok(Val::ACC(Box::new(Val::NULL))),
                Some(acc) => Ok(Val::ACC(Box::new(*acc.clone()))),
            },
            _ => Err(FqError::exe("min() only works for number types")),
        }
    }

    // expressions

    fn plus(&self, other: &Val) -> Result<Val, FqError> {
        match (self, other) {
            (Val::INT(val), Val::INT(other)) => Ok(Val::INT(*val + *other)),
            (Val::INT(val), Val::FLOAT(other)) => Ok(Val::FLOAT((*val as f32) + *other)),
            (Val::FLOAT(val), Val::INT(other)) => Ok(Val::FLOAT(*val + (*other as f32))),
            (Val::FLOAT(val), Val::FLOAT(other)) => Ok(Val::FLOAT(*val + *other)),
            (Val::NULL, _) | (_, Val::NULL) => Ok(Val::NULL),
            _ => Err(FqError::exe("+ can only be used with int and float values")),
        }
    }

    fn minus(&self, other: &Val) -> Result<Val, FqError> {
        match (self, other) {
            (Val::INT(val), Val::INT(other)) => Ok(Val::INT(*val - *other)),
            (Val::INT(val), Val::FLOAT(other)) => Ok(Val::FLOAT((*val as f32) - *other)),
            (Val::FLOAT(val), Val::INT(other)) => Ok(Val::FLOAT(*val - (*other as f32))),
            (Val::FLOAT(val), Val::FLOAT(other)) => Ok(Val::FLOAT(*val - *other)),
            (Val::NULL, _) | (_, Val::NULL) => Ok(Val::NULL),
            _ => Err(FqError::exe("- can only be used with int and float values")),
        }
    }

    fn mul(&self, other: &Val) -> Result<Val, FqError> {
        match (self, other) {
            (Val::INT(val), Val::INT(other)) => Ok(Val::INT(*val * *other)),
            (Val::INT(val), Val::FLOAT(other)) => Ok(Val::FLOAT((*val as f32) * *other)),
            (Val::FLOAT(val), Val::INT(other)) => Ok(Val::FLOAT(*val * (*other as f32))),
            (Val::FLOAT(val), Val::FLOAT(other)) => Ok(Val::FLOAT(*val * *other)),
            (Val::NULL, _) | (_, Val::NULL) => Ok(Val::NULL),
            _ => Err(FqError::exe("* can only be used with int and float values")),
        }
    }

    fn div(&self, other: &Val) -> Result<Val, FqError> {
        if let Val::INT(0) | Val::FLOAT(0.0) = other {
            return Err(FqError::exe("division by 0"));
        }
        match (self, other) {
            (Val::INT(val), Val::INT(other)) => Ok(Val::INT(*val / *other)),
            (Val::INT(val), Val::FLOAT(other)) => Ok(Val::FLOAT((*val as f32) / *other)),
            (Val::FLOAT(val), Val::INT(other)) => Ok(Val::FLOAT(*val / (*other as f32))),
            (Val::FLOAT(val), Val::FLOAT(other)) => Ok(Val::FLOAT(*val / *other)),
            (Val::NULL, _) | (_, Val::NULL) => Ok(Val::NULL),
            _ => Err(FqError::exe("/ can only be used with int and float values")),
        }
    }

    fn lt(&self, other: &Val) -> Result<Val, FqError> {
        match (self, other) {
            (Val::INT(val), Val::INT(other)) => Ok(Val::BOOL(*val < *other)),
            (Val::INT(val), Val::FLOAT(other)) => Ok(Val::BOOL((*val as f32) < *other)),
            (Val::FLOAT(val), Val::INT(other)) => Ok(Val::BOOL(*val < (*other as f32))),
            (Val::FLOAT(val), Val::FLOAT(other)) => Ok(Val::BOOL(*val < *other)),
            (Val::STR(val), Val::STR(other)) => Ok(Val::BOOL(*val < *other)),
            _ => Err(FqError::exe(">, >=, <, <= can be used with the following pairs (int, int), (int, float), (float, float), and (str, str)")),
        }
    }

    fn eq(&self, other: &Val) -> Result<Val, FqError> {
        match (self, other) {
            (Val::INT(val), Val::INT(other)) => Ok(Val::BOOL(*val == *other)),
            (Val::INT(val), Val::FLOAT(other)) => Ok(Val::BOOL((*val as f32) == *other)),
            (Val::FLOAT(val), Val::INT(other)) => Ok(Val::BOOL(*val == (*other as f32))),
            (Val::FLOAT(val), Val::FLOAT(other)) => Ok(Val::BOOL(*val == *other)),
            (Val::STR(val), Val::STR(other)) => Ok(Val::BOOL(*val == *other)),
            (Val::BOOL(val), Val::BOOL(other)) => Ok(Val::BOOL(*val == *other)),
            _ => Err(FqError::exe("==, != ca be used with the following pairs (int, int), (int, float), (float, float), and (str, str)")),
        }
    }

    fn gt(&self, other: &Val) -> Result<Val, FqError> {
        other.lt(self)
    }

    fn le(&self, other: &Val) -> Result<Val, FqError> {
        let result = self.lt(other)?;
        if let Val::BOOL(false) = result {
            return self.eq(other);
        }
        Ok(result)
    }

    fn ge(&self, other: &Val) -> Result<Val, FqError> {
        let result = self.gt(other)?;
        if let Val::BOOL(false) = result {
            return self.eq(other);
        }
        Ok(result)
    }

    fn ne(&self, other: &Val) -> Result<Val, FqError> {
        if let Val::BOOL(false) = self.eq(other)? {
            return Ok(Val::BOOL(true));
        } else {
            return Ok(Val::BOOL(false));
        }
    }
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Val::INT(val) => write!(f, "{}", val.to_string()),
            Val::FLOAT(val) => write!(f, "{}", val.to_string()),
            Val::BOOL(val) => write!(f, "{}", val.to_string()),
            Val::STR(val) => write!(f, "{}", val.to_string()),
            Val::ACC(val) => write!(f, "{}", val.to_string()),
            Val::NULL => write!(f, " "),
        }
    }
}

impl Clone for Val {
    fn clone(&self) -> Val {
        match self {
            Val::INT(val) => Val::INT(*val),
            Val::FLOAT(val) => Val::FLOAT(*val),
            Val::BOOL(val) => Val::BOOL(*val),
            Val::STR(val) => Val::STR(String::from(val)),
            Val::ACC(val) => Val::ACC(Box::new(*val.clone())),
            Val::NULL => Val::NULL,
        }
    }
}

pub trait Expr {
    fn eval(&self, row: &Vec<String>, acc: &Option<Box<Val>>) -> Result<Val, FqError>;

    // todo: design to be improved
    fn is_star(&self) -> bool {
        false
    }

    // oh well
    fn finish(&self, acc: Val, _nrows: i32) -> Result<Val, FqError> {
        Ok(acc)
    }
}

/// Represents an integer value.
pub struct IntConst {
    val: i32,
}

impl IntConst {
    pub fn new(val: i32) -> IntConst {
        IntConst { val }
    }
}

impl Expr for IntConst {
    fn eval(&self, _row: &Vec<String>, _acc: &Option<Box<Val>>) -> Result<Val, FqError> {
        Ok(Val::INT(self.val))
    }
}

pub struct BoolConst {
    val: bool,
}

impl BoolConst {
    pub fn new(val: bool) -> BoolConst {
        BoolConst { val }
    }
}

impl Expr for BoolConst {
    fn eval(&self, _row: &Vec<String>, _acc: &Option<Box<Val>>) -> Result<Val, FqError> {
        Ok(Val::BOOL(self.val))
    }
}

/// Represents a column expression, e.g., @1
pub struct ColRef {
    val: u32,
}

impl ColRef {
    pub fn new(val: u32) -> ColRef {
        ColRef { val }
    }
}

impl Expr for ColRef {
    fn eval(&self, row: &Vec<String>, _acc: &Option<Box<Val>>) -> Result<Val, FqError> {
        if (self.val as usize) >= row.len() {
            return Err(FqError::exe(format!(
                "index out of bounds: number of columns is {} but the column reference is {}",
                row.len(),
                self.val
            )));
        }
        Ok(Val::STR(row[self.val as usize].to_string()))
    }
}

pub struct StarConst;

impl StarConst {
    pub fn new() -> StarConst {
        StarConst
    }
}

impl Expr for StarConst {
    fn eval(&self, _row: &Vec<String>, _acc: &Option<Box<Val>>) -> Result<Val, FqError> {
        Err(FqError::exe("Never eval *"))
    }

    fn is_star(&self) -> bool {
        true
    }
}

pub struct StrConst {
    val: String,
}

impl StrConst {
    pub fn new(val: String) -> StrConst {
        StrConst { val }
    }
}

impl Expr for StrConst {
    fn eval(&self, _row: &Vec<String>, _acc: &Option<Box<Val>>) -> Result<Val, FqError> {
        Ok(Val::STR(String::from(&self.val)))
    }
}

pub struct FloatConst {
    val: f32,
}

impl FloatConst {
    pub fn new(val: f32) -> FloatConst {
        FloatConst { val }
    }
}

impl Expr for FloatConst {
    fn eval(&self, _row: &Vec<String>, _acc: &Option<Box<Val>>) -> Result<Val, FqError> {
        Ok(Val::FLOAT(self.val))
    }
}

pub enum Bop {
    GT,
    LT,
    EQ,
    GE,
    LE,
    NE,
    PLUS,
    MINUS,
    MUL,
    DIV,
}

pub struct BinExpr {
    op: Bop,
    left: Rc<dyn Expr>,
    right: Rc<dyn Expr>,
}

impl BinExpr {
    pub fn new(op: Bop, left: Rc<dyn Expr>, right: Rc<dyn Expr>) -> BinExpr {
        BinExpr { op, left, right }
    }
}

impl Expr for BinExpr {
    fn eval(&self, row: &Vec<String>, acc: &Option<Box<Val>>) -> Result<Val, FqError> {
        let left_val = self.left.eval(row, acc)?;
        let right_val = self.right.eval(row, acc)?;
        match self.op {
            Bop::GT => left_val.gt(&right_val),
            Bop::LT => left_val.lt(&right_val),
            Bop::EQ => left_val.eq(&right_val),
            Bop::LE => left_val.le(&right_val),
            Bop::GE => left_val.ge(&right_val),
            Bop::NE => left_val.ne(&right_val),
            Bop::PLUS => left_val.plus(&right_val),
            Bop::MINUS => left_val.minus(&right_val),
            Bop::MUL => left_val.mul(&right_val),
            Bop::DIV => left_val.div(&right_val),
        }
    }
}

pub struct FuncCall {
    name: String,
    // todo: support multiple arguments
    args: Rc<dyn Expr>,
}

impl FuncCall {
    pub fn new(name: &str, args: Rc<dyn Expr>) -> FuncCall {
        FuncCall {
            name: name.to_string(),
            args,
        }
    }
}

impl Expr for FuncCall {
    fn eval(&self, row: &Vec<String>, acc: &Option<Box<Val>>) -> Result<Val, FqError> {
        match self.name.as_str() {
            ABS_FUNC => self.args.eval(row, acc)?.abs(),
            UPPER_FUNC => self.args.eval(row, acc)?.upper(),
            LOWER_FUNC => self.args.eval(row, acc)?.lower(),
            LENGTH_FUNC => self.args.eval(row, acc)?.length(),
            REV_FUNC => self.args.eval(row, acc)?.rev(),
            SIGN_FUNC => self.args.eval(row, acc)?.sign(),
            CEIL_FUNC => self.args.eval(row, acc)?.ceil(),
            FLOOR_FUNC => self.args.eval(row, acc)?.floor(),
            ROUND_FUNC => self.args.eval(row, acc)?.round(),
            COS_FUNC => self.args.eval(row, acc)?.cos(),
            SIN_FUNC => self.args.eval(row, acc)?.sin(),
            // aggragate
            SUM_FUNC => self.args.eval(row, acc)?.sum(acc),
            COUNT_FUNC => self.args.eval(row, acc)?.count(acc),
            MAX_FUNC => self.args.eval(row, acc)?.max(acc),
            MIN_FUNC => self.args.eval(row, acc)?.min(acc),
            AVG_FUNC => self.args.eval(row, acc)?.sum(acc),
            _ => Err(FqError::exe(format!("Unsupported function {}", self.name))),
        }
    }

    fn finish(&self, acc: Val, nrows: i32) -> Result<Val, FqError> {
        match self.name.as_str() {
            AVG_FUNC => acc.div(&Val::INT(nrows)),
            _ => Ok(acc),
        }
    }
}

pub struct IntCast {
    exp: Rc<dyn Expr>,
}

impl IntCast {
    pub fn new(exp: Rc<dyn Expr>) -> IntCast {
        IntCast { exp }
    }
}

impl Expr for IntCast {
    fn eval(&self, row: &Vec<String>, acc: &Option<Box<Val>>) -> Result<Val, FqError> {
        // Cast should only be used on column references, so we always
        // expect a string.
        match self.exp.eval(row, acc)? {
            Val::STR(val) if val == "" => Ok(Val::NULL),
            Val::STR(val) => match val.parse::<i32>() {
                Ok(num) => Ok(Val::INT(num)),
                Err(_) => Err(FqError::exe(format!("Cannot cast {} to int", val))),
            },
            _ => Err(FqError::exe("Cast can be used only on column references")),
        }
    }
}

pub struct FloatCast {
    exp: Rc<dyn Expr>,
}

impl FloatCast {
    pub fn new(exp: Rc<dyn Expr>) -> FloatCast {
        FloatCast { exp }
    }
}

impl Expr for FloatCast {
    fn eval(&self, row: &Vec<String>, acc: &Option<Box<Val>>) -> Result<Val, FqError> {
        match self.exp.eval(row, acc)? {
            Val::STR(val) if val == "" => Ok(Val::NULL),
            Val::STR(val) => match val.parse::<f32>() {
                Ok(num) => Ok(Val::FLOAT(num)),
                Err(_) => Err(FqError::exe(format!("Cannot cast {} to float", val))),
            },
            _ => Err(FqError::exe("Cast can be used only on column references")),
        }
    }
}

pub struct BoolCast {
    exp: Rc<dyn Expr>,
}

impl BoolCast {
    pub fn new(exp: Rc<dyn Expr>) -> BoolCast {
        BoolCast { exp }
    }
}

impl Expr for BoolCast {
    fn eval(&self, row: &Vec<String>, acc: &Option<Box<Val>>) -> Result<Val, FqError> {
        match self.exp.eval(row, acc)? {
            Val::STR(val) if val == "" => Ok(Val::NULL),
            Val::STR(val) => match val.parse::<bool>() {
                Ok(val) => Ok(Val::BOOL(val)),
                Err(_) => Err(FqError::exe(format!("Cannot cast {} to bool", val))),
            },
            _ => Err(FqError::exe("Cast can be used only on column references")),
        }
    }
}

pub struct StrCast {
    exp: Rc<dyn Expr>,
}

impl StrCast {
    pub fn new(exp: Rc<dyn Expr>) -> StrCast {
        StrCast { exp }
    }
}

impl Expr for StrCast {
    fn eval(&self, row: &Vec<String>, acc: &Option<Box<Val>>) -> Result<Val, FqError> {
        match self.exp.eval(row, acc)? {
            Val::STR(val) if val == "" => Ok(Val::NULL),
            Val::STR(val) => Ok(Val::STR(val)),
            _ => Err(FqError::exe("Cast can be used only on column references")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_bool() {
        let exp = BoolConst::new(true);
        match exp.eval(&vec![], &None) {
            Ok(Val::BOOL(val)) => assert!(val),
            _ => panic!("Incorrect bool eval"),
        }
    }

    #[test]
    fn eval_int() {
        let exp = IntConst::new(33);
        match exp.eval(&vec![], &None) {
            Ok(Val::INT(val)) => assert_eq!(33, val),
            _ => panic!("Incorrect int eval"),
        }
    }

    #[test]
    fn eval_string() {
        let exp = StrConst::new(String::from("best string"));
        match exp.eval(&vec![], &None) {
            Ok(Val::STR(val)) => assert_eq!("best string", val),
            _ => panic!("Incorrect string eval"),
        }
    }

    #[test]
    fn eval_relation() {
        let exp = BinExpr::new(
            Bop::GT,
            Rc::new(IntConst::new(55)),
            Rc::new(FloatConst::new(60.0)),
        );

        match exp.eval(&vec![], &None) {
            Ok(Val::BOOL(val)) => assert!(!val),
            _ => panic!("Incorrect relation eval"),
        }

        let exp = BinExpr::new(
            Bop::LT,
            Rc::new(IntConst::new(55)),
            Rc::new(FloatConst::new(60.0)),
        );

        match exp.eval(&vec![], &None) {
            Ok(Val::BOOL(val)) => assert!(val),
            _ => panic!("Incorrect relation eval"),
        }
    }

    #[test]
    fn eval_int_cast() {
        let exp = IntCast::new(Rc::new(ColRef::new(1)));
        match exp.eval(&vec![String::from("one"), String::from("123")], &None) {
            Ok(Val::INT(val)) => assert_eq!(123, val),
            _ => panic!("Incorrect int cast eval"),
        }
    }

    #[test]
    fn eval_float_cast() {
        let exp = FloatCast::new(Rc::new(ColRef::new(0)));
        match exp.eval(&vec![String::from("123.00"), String::from("543.0")], &None) {
            Ok(Val::FLOAT(val)) => assert!((123.00 - val).abs() < f32::EPSILON),
            _ => panic!("Incorrect float cast eval"),
        }
    }

    #[test]
    #[should_panic(expected = "Execution error: division by 0")]
    fn eval_div_by_zero() {
        let exp = BinExpr::new(
            Bop::DIV,
            Rc::new(IntConst::new(55)),
            Rc::new(IntConst::new(0)),
        );
        exp.eval(&vec![], &None)
            .unwrap_or_else(|err| panic!("{err}"));
    }

    #[test]
    #[should_panic(expected = "Execution error: index out of bounds")]
    fn eval_column_ref_out_of_bounds() {
        let exp = ColRef::new(100);
        exp.eval(&vec![], &None)
            .unwrap_or_else(|err| panic!("{err}"));
    }

    #[test]
    #[should_panic(expected = "Execution error: Cannot cast abc to int")]
    fn eval_int_cast_error() {
        let exp = IntCast::new(Rc::new(ColRef::new(0)));
        exp.eval(&vec![String::from("abc")], &None)
            .unwrap_or_else(|err| panic!("{err}"));
    }

    #[test]
    #[should_panic(expected = "Execution error: Cannot cast abc to float")]
    fn eval_float_cast_error() {
        let exp = FloatCast::new(Rc::new(ColRef::new(0)));
        exp.eval(&vec![String::from("abc")], &None)
            .unwrap_or_else(|err| panic!("{err}"));
    }

    #[test]
    #[should_panic(expected = "Execution error: Cannot cast abc to bool")]
    fn eval_bool_cast_error() {
        let exp = BoolCast::new(Rc::new(ColRef::new(0)));
        exp.eval(&vec![String::from("abc")], &None)
            .unwrap_or_else(|err| panic!("{err}"));
    }

    #[test]
    fn eval_abs_func() {
        let exp = FuncCall::new(ABS_FUNC, Rc::new(IntCast::new(Rc::new(ColRef::new(0)))));
        match exp.eval(&vec![String::from("-3"), String::from("abc")], &None) {
            Ok(Val::INT(val)) => assert_eq!(3, val),
            _ => panic!("abs() errors"),
        }
    }

    #[test]
    fn eval_upper_func() {
        let exp = FuncCall::new(
            UPPER_FUNC,
            Rc::new(StrConst::new(String::from("something"))),
        );
        match exp.eval(&vec![], &None) {
            Ok(Val::STR(val)) => assert_eq!("SOMETHING", val),
            _ => panic!("upper() errors"),
        }
    }

    #[test]
    fn eval_lower_func() {
        let exp = FuncCall::new(
            LOWER_FUNC,
            Rc::new(StrConst::new(String::from("Something"))),
        );
        match exp.eval(&vec![], &None) {
            Ok(Val::STR(val)) => assert_eq!("something", val),
            _ => panic!("lower() errors"),
        }
    }

    #[test]
    fn eval_length_func() {
        let exp = FuncCall::new(
            LENGTH_FUNC,
            Rc::new(StrConst::new(String::from("FQL Tutorial"))),
        );
        match exp.eval(&vec![], &None) {
            Ok(Val::INT(val)) => assert_eq!(12, val),
            _ => panic!("length() errors"),
        }
    }

    #[test]
    fn eval_rev_func() {
        let exp = FuncCall::new(REV_FUNC, Rc::new(StrConst::new(String::from("something"))));
        match exp.eval(&vec![], &None) {
            Ok(Val::STR(val)) => assert_eq!("gnihtemos", val),
            _ => panic!("rev() errors"),
        }
    }

    #[test]
    fn eval_sign_func() {
        let exp = FuncCall::new(SIGN_FUNC, Rc::new(IntConst::new(33)));
        match exp.eval(&vec![], &None) {
            Ok(Val::INT(1)) => (),
            _ => panic!("sign(1) errors"),
        }

        let exp = FuncCall::new(SIGN_FUNC, Rc::new(IntConst::new(-33)));
        match exp.eval(&vec![], &None) {
            Ok(Val::INT(-1)) => (),
            _ => panic!("sign(-1) errors"),
        }

        let exp = FuncCall::new(SIGN_FUNC, Rc::new(IntConst::new(0)));
        match exp.eval(&vec![], &None) {
            Ok(Val::INT(0)) => (),
            _ => panic!("sign(0) errors"),
        }
    }

    #[test]
    fn eval_ceil_func() {
        let exp = FuncCall::new(CEIL_FUNC, Rc::new(FloatConst::new(3.44)));
        match exp.eval(&vec![], &None) {
            Ok(Val::FLOAT(val)) => assert!((val - 4.0).abs() < f32::EPSILON),
            _ => panic!("ceil() errors"),
        }
    }

    #[test]
    fn eval_floor_func() {
        let exp = FuncCall::new(FLOOR_FUNC, Rc::new(FloatConst::new(3.44)));
        match exp.eval(&vec![], &None) {
            Ok(Val::FLOAT(val)) => assert!((val - 3.0).abs() < f32::EPSILON),
            _ => panic!("floor() errors"),
        }
    }

    #[test]
    fn eval_round_func() {
        let exp = FuncCall::new(ROUND_FUNC, Rc::new(FloatConst::new(3.44)));
        match exp.eval(&vec![], &None) {
            Ok(Val::FLOAT(val)) => assert!((val - 3.0).abs() < f32::EPSILON),
            _ => panic!("round() errors"),
        }
    }

    #[test]
    fn eval_cos_func() {
        let exp = FuncCall::new(COS_FUNC, Rc::new(FloatConst::new(3.4)));
        match exp.eval(&vec![], &None) {
            Ok(Val::FLOAT(val)) => assert!((val + 0.9667982).abs() < f32::EPSILON),
            _ => panic!("cos() errors"),
        }
    }

    #[test]
    fn eval_sin_func() {
        let exp = FuncCall::new(SIN_FUNC, Rc::new(FloatConst::new(3.4)));
        match exp.eval(&vec![], &None) {
            Ok(Val::FLOAT(val)) => assert!((val + 0.2555412).abs() < f32::EPSILON, "{val}"),
            _ => panic!("sin() errors"),
        }
    }

    #[test]
    fn eval_sum_func() {
        let exp = FuncCall::new(SUM_FUNC, Rc::new(IntCast::new(Rc::new(ColRef::new(0)))));
        let mut acc = None;

        match exp.eval(&vec![String::from("22")], &acc) {
            Ok(Val::ACC(b)) => acc = Some(Box::new(*b)),
            _ => panic!("sum() 1st call error"),
        }

        match exp.eval(&vec![String::from("33")], &acc) {
            Ok(Val::ACC(b)) => acc = Some(Box::new(*b)),
            _ => panic!("sum() 2nd call error"),
        }

        match acc {
            Some(boxed) => match *boxed {
                Val::INT(val) => assert_eq!(55, val),
                _ => panic!("Unexpected variant"),
            },
            None => panic!("sum() total error"),
        }
    }

    #[test]
    fn eval_count_func() {
        let exp = FuncCall::new(COUNT_FUNC, Rc::new(IntCast::new(Rc::new(ColRef::new(0)))));
        let mut acc = None;

        match exp.eval(&vec![String::from("3")], &acc) {
            Ok(Val::ACC(b)) => acc = Some(Box::new(*b)),
            _ => panic!("count() 1st error"),
        }

        match exp.eval(&vec![String::from("5")], &acc) {
            Ok(Val::ACC(b)) => acc = Some(Box::new(*b)),
            _ => panic!("count() 2nd error"),
        }

        match acc {
            Some(boxed) => match *boxed {
                Val::INT(val) => assert_eq!(2, val),
                _ => panic!("Unexpected variant"),
            },
            None => panic!("count() total error"),
        }
    }

    #[test]
    fn eval_max_func() {
        let exp = FuncCall::new(MAX_FUNC, Rc::new(IntCast::new(Rc::new(ColRef::new(0)))));
        let mut acc = None;

        match exp.eval(&vec![String::from("3")], &acc) {
            Ok(Val::ACC(boxed)) => acc = Some(Box::new(*boxed)),
            _ => panic!("max() 1st eerror"),
        }

        match exp.eval(&vec![String::from("20")], &acc) {
            Ok(Val::ACC(boxed)) => acc = Some(Box::new(*boxed)),
            _ => panic!("max() 2nd error"),
        }

        match acc {
            Some(boxed) => match *boxed {
                Val::INT(val) => assert_eq!(20, val),
                _ => panic!("Unexpected variant"),
            },
            None => panic!("max() total error"),
        }
    }

    #[test]
    fn eval_min_func() {
        let exp = FuncCall::new(MIN_FUNC, Rc::new(IntCast::new(Rc::new(ColRef::new(0)))));
        let mut acc = None;

        match exp.eval(&vec![String::from("3")], &acc) {
            Ok(Val::ACC(boxed)) => acc = Some(Box::new(*boxed)),
            _ => panic!("min() 1st error"),
        }

        match exp.eval(&vec![String::from("20")], &acc) {
            Ok(Val::ACC(boxed)) => acc = Some(Box::new(*boxed)),
            _ => panic!("min() 2nd error"),
        }

        match acc {
            Some(boxed) => match *boxed {
                Val::INT(val) => assert_eq!(3, val),
                _ => panic!("Unexpected variant"),
            },
            None => panic!("min() total error"),
        }
    }
}
