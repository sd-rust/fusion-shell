// Copyright (C) 2016  Sandeep Datta

use asg::*;
use std::env;
use std::path::PathBuf;
use std::io;
use std::fmt;

#[derive(Debug)]
pub enum StreamError {
    Io(io::Error),
    CommandArity(usize, usize, usize), // (min, max, Found)
    //TODO: Rename to ArgStreamLen
    ArgStreamArity(usize, usize, usize, usize), //(Arg Index, min, max, Found)
    ArgType(usize),
    BadCommand,
}

impl fmt::Display for StreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StreamError::Io(ref err) => write!(f, "{}", err),
            StreamError::CommandArity(min, max, found) if min == max
                => write!(f, "Incorrect command arity. Expected: {}, found: {}."
                           , min, found),
            StreamError::CommandArity(min, max, found) 
                => write!(f, "Incorrect command arity. Minimum expected {}, maximum expected: {} found: {}."
                           , min, max, found),
            StreamError::ArgStreamArity(arg_index, min, max, found) if min == max
                => write!(f, "Incorrect argument stream arity for arg {}. Expected {}, found {}."
                           , arg_index, min, found),
            StreamError::ArgStreamArity(arg_index, min, max, found)
                => write!(f, "Incorrect argument stream arity for arg {}. Minimum expected: {}, maximum expected: {}, found {}."
                           , arg_index, min, max, found),
            StreamError::ArgType(arg_index) => write!(f, "Incorrect argument type for arg {}", arg_index),
            StreamError::BadCommand => write!(f, "Bad command"),
        }
    }
}

pub type Stream = Vec<Result<StreamElement, StreamError>>;

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

pub fn pwd(arg_streams: Vec<Stream>) -> Stream {

    let result = match arg_streams[..] {
        [] => {
            env::current_dir()
                .map(From::from)
                .map_err(From::from)
        }
        _ => Err(StreamError::CommandArity(0, 0, arg_streams.len())),
    };

    vec![result]
}

pub fn cd(arg_streams: Vec<Stream>) -> Stream {
    if arg_streams.len() != 1 {
        return vec![Err(StreamError::CommandArity(1, 1, arg_streams.len()))]
    }

    let arg_stream = &arg_streams[0];

    if arg_stream.len() != 1 {
        return vec![Err(StreamError::ArgStreamArity(0, 1, 1, arg_stream.len()))]
    }

    let arg = &arg_stream[0];

    let result = match arg {
        &Ok(StreamElement::Path(ref p)) => {
            env::set_current_dir(p)
                .map(From::from)
                .map_err(From::from)
        }
        _ => Err(StreamError::ArgType(0)),
    };

    vec![result]
}

pub fn exit(arg_streams: Vec<Stream>) -> Stream {
    let result = match arg_streams[..] {
        [ref arg_stream] => match arg_stream[..] {
            [Ok(StreamElement::Int(exit_code))] => Ok(StreamElement::Exit(exit_code as i32)),
            _ => Err(StreamError::ArgType(0)),
        },
        [] => Ok(StreamElement::Exit(0)),
        _ => Err(StreamError::CommandArity(0, 1, arg_streams.len())),
    };

    vec![result]   
}
