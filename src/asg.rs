// Copyright (C) 2016  Sandeep Datta

// Abstract Semantic Graph

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Expression {
    Int(i64),
    Str(String),
    CommandApp(String, Vec<Expression>), // Name, Args
}
