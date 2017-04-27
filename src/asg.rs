// Copyright (C) 2016  Sandeep Datta

// Abstract Semantic Graph

use std::fmt;
use std::path::PathBuf;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Primitive {
    None,
    Int(i64),
    Str(String),
    Path(PathBuf), // Exit(i32), // TODO: rename to Error?
}

impl fmt::Display for Primitive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Primitive::None => write!(f, ""),
            Primitive::Int(ref val) => write!(f, "{}", val),
            Primitive::Str(ref val) => write!(f, "{}", val),
            Primitive::Path(ref val) => write!(f, "{}", val.display()),
            // Primitive::Exit(ecode) => write!(f, "<exit {}>", ecode),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FunctionApplication {
    pub name: String,
    pub args: Vec<Expression>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FunctionParam {
    pub name: String,
    pub param_type: String,
}

// TODO: Rename Expression to Statement
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Expression {
    Value(Primitive),
    FunctionDefinition {
        name: String,
        params: Vec<FunctionParam>,
        body: Vec<Expression>,
    },
}

impl From<Primitive> for Expression {
    fn from(pv: Primitive) -> Expression {
        Expression::Value(pv)
    }
}
