// Copyright (C) 2016  Sandeep Datta

// Abstract Semantic Graph

use std::fmt;
use std::path::PathBuf;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum StreamElement {
    None,
    Int(i64),
    Str(String),
    Path(PathBuf),
    Exit(i32), // TODO: rename to Error?
}

impl fmt::Display for StreamElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StreamElement::None => write!(f, ""),
            StreamElement::Int(ref val) => write!(f, "{}", val),
            StreamElement::Str(ref val) => write!(f, "{}", val),
            StreamElement::Path(ref val) => write!(f, "{}", val.display()),
            StreamElement::Exit(ecode) => write!(f, "<exit {}>", ecode),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CommandApplication {
    pub name: String,
    pub args: Vec<Expression>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Expression {
    Value(StreamElement),
    Command(CommandApplication),
    //Map(CommandApplication, CommandApplication),
}

impl From<StreamElement> for Expression {
    fn from(pv: StreamElement) -> Expression {
        Expression::Value(pv)
    }
}