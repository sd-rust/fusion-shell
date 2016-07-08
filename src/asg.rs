// Copyright (C) 2016  Sandeep Datta

// Abstract Semantic Graph

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CommandApplication {
    pub name: String,
    pub args: Vec<Expression>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Expression {
    Int(i64),
    Str(String),
    //TODO: Rename to Command
    Command(CommandApplication),
    Map(CommandApplication, CommandApplication),
}
