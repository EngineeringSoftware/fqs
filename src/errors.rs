use std::{error::Error, fmt};

#[derive(Debug)]
pub enum FqError {
    Syntax(String),
    Parse(String),
    Semantics(String),
    Exe(String),
    Internal(String),
}

impl FqError {
    pub fn syntax<S: AsRef<str>>(msg: S) -> FqError {
        FqError::Syntax(format!("Syntax error: {}", msg.as_ref()))
    }

    pub fn parse<S: AsRef<str>>(msg: S) -> FqError {
        FqError::Parse(format!("Parse error: {}", msg.as_ref()))
    }

    pub fn semantics<S: AsRef<str>>(msg: S) -> FqError {
        FqError::Semantics(format!("Semantics error: {}", msg.as_ref()))
    }

    pub fn exe<S: AsRef<str>>(msg: S) -> FqError {
        FqError::Exe(format!("Execution error: {}", msg.as_ref()))
    }

    pub fn internal<S: AsRef<str>>(msg: S) -> FqError {
        FqError::Internal(format!("Internal error {}", msg.as_ref()))
    }
}

impl fmt::Display for FqError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FqError::Parse(s) => write!(f, "{}", s),
            FqError::Syntax(s) => write!(f, "{}", s),
            FqError::Semantics(s) => write!(f, "{}", s),
            FqError::Exe(s) => write!(f, "{}", s),
            FqError::Internal(s) => write!(f, "{}", s),
        }
    }
}

impl Error for FqError {}
