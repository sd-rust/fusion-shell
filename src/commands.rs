// Copyright (C) 2016  Sandeep Datta

use asg::*;
use std::env;
use std::path::PathBuf;
use std::io;
use std::result;
use std::fmt;

#[derive(Debug)]
pub enum StreamError {
    Io(io::Error),
    MalformedCommand,
    BadCommand,
}

impl fmt::Display for StreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StreamError::Io(ref err) => write!(f, "{}", err),
            StreamError::MalformedCommand => write!(f, "Malformed command"),
            StreamError::BadCommand => write!(f, "Bad command"),
        }
    }
}

pub type Result<T> = result::Result<T, StreamError>;

impl From<io::Error> for StreamError {
    fn from(err: io::Error) -> StreamError {
        StreamError::Io(err)
    }
}

impl From<PathBuf> for StreamElement {
    fn from(path_buff: PathBuf) -> StreamElement {
        StreamElement::Path(path_buff)
    }
}

impl From<()> for StreamElement {
    fn from(_: ()) -> StreamElement {
        StreamElement::None
    }
}

pub fn pwd(args: Vec<StreamElement>) -> Result<StreamElement> {
    match args[..] {
        [] => {
            env::current_dir()
                .map(From::from)
                .map_err(From::from)
        }
        _ => Err(StreamError::MalformedCommand),
    }
}

pub fn cd(args: Vec<StreamElement>) -> Result<StreamElement> {
    match args[..] {
        [StreamElement::Path(ref p)] => {
            env::set_current_dir(p)
                .map(From::from)
                .map_err(From::from)
        }
        _ => Err(StreamError::MalformedCommand),
    }
}

pub fn exit(args: Vec<StreamElement>) -> Result<StreamElement> {
    match args[..] {
        [StreamElement::Int(exit_code)] => Ok(StreamElement::Exit(exit_code as i32)),
        [] => Ok(StreamElement::Exit(0)),
        _ => Err(StreamError::MalformedCommand),
    }
}
