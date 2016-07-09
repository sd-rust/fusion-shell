// Copyright (C) 2016  Sandeep Datta

use asg::*;
use std::env;
use std::path::PathBuf;
use std::process;
use std::io;
use std::result;
use std::fmt;

#[derive(Debug)]
pub enum PipeError {
    Io(io::Error),
    MalformedCommand,
    BadCommand,
}

impl fmt::Display for PipeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PipeError::Io(ref err) => write!(f, "{}", err),
            PipeError::MalformedCommand => write!(f, "Malformed command"),
            PipeError::BadCommand => write!(f, "Bad command"),
        }
    }
}

pub type Result<T> = result::Result<T, PipeError>;

impl From<io::Error> for PipeError {
    fn from(err: io::Error) -> PipeError {
        PipeError::Io(err)
    }
}

impl From<PathBuf> for PipeValue {
    fn from(path_buff: PathBuf) -> PipeValue {
        PipeValue::Path(path_buff)
    }
}

impl From<()> for PipeValue {
    fn from(_: ()) -> PipeValue {
        PipeValue::None
    }
}

pub fn pwd(args: Vec<PipeValue>) -> Result<PipeValue> {
    match args[..] {
        [] => {
            env::current_dir()
                .map(From::from)
                .map_err(From::from)
        }
        _ => Err(PipeError::MalformedCommand),
    }
}

pub fn cd(args: Vec<PipeValue>) -> Result<PipeValue> {
    match args[..] {
        [PipeValue::Path(ref p)] => {
            env::set_current_dir(p)
                .map(From::from)
                .map_err(From::from)
        }
        _ => Err(PipeError::MalformedCommand),
    }
}

pub fn exit(args: Vec<PipeValue>) -> Result<PipeValue> {
    match args[..] {
        [PipeValue::Int(exit_code)] => process::exit(exit_code as i32),
        [] => process::exit(0),
        _ => Err(PipeError::MalformedCommand),
    }
}
