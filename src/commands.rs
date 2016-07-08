// Copyright (C) 2016  Sandeep Datta

use asg::*;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use std::io;
use std::result;

#[derive(Debug)]
pub enum PipeError {
    Io(io::Error),
    MalformedCommand,
}

pub type Result<T> = result::Result<T, PipeError>;

impl From<io::Error> for PipeError {
    fn from(err: io::Error) -> PipeError {
        PipeError::Io(err)
    }
}

pub fn pwd(args: Vec<Expression>) -> Result<PathBuf> {
    match args[..] {
        [] => env::current_dir().map_err(|err| From::from(err)),
        _ => Err(PipeError::MalformedCommand),
    }
}

pub fn cd(args: Vec<Expression>) -> Result<()> {
    match args[..] {
        [Expression::Str(ref s)] => {
            let root = Path::new(s);
            env::set_current_dir(&root).map_err(|err| From::from(err))
        }
        _ => Err(PipeError::MalformedCommand),
    }
}

pub fn exit(args: Vec<Expression>) {
    match args[..] {
        [Expression::Int(exit_code)] => process::exit(exit_code as i32),
        [] => process::exit(0),
        _ => print_err_ln!("Warning: Ignoring malformed command."),
    }
}