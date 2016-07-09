// Copyright (C) 2016  Sandeep Datta

// Abstract Semantic Graph

use std::path::PathBuf;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum PipeValue {
    None,
    Int(i64),
    Str(String),
    Path(PathBuf),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CommandApplication {
    pub name: String,
    pub args: Vec<Expression>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Expression {
    Value(PipeValue),
    Command(CommandApplication),
    //Map(CommandApplication, CommandApplication),
}

impl From<PipeValue> for Expression {
    fn from(pv: PipeValue) -> Expression {
        Expression::Value(pv)
    }
}