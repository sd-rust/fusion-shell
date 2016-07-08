// Copyright (C) 2016  Sandeep Datta

use asg::*;
use utils;
use std::env;
use std::path::Path;
use std::process;

pub fn pwd(args: Vec<Expression>) {
    match args[..] {
        [] => utils::print_curdir(),
        _ => print_err_ln!("Warning: Ignoring malformed command."),
    }
}

pub fn cd(args: Vec<Expression>) {
    match args[..] {
        [Expression::Str(ref s)] => {
            let root = Path::new(s);
            if let Err(err) = env::set_current_dir(&root) {
                print_err_ln!("Error: Could not set current directory. {}.", err);
            }
        }
        [] => utils::print_curdir(),
        _ => print_err_ln!("Warning: Ignoring malformed command."),
    }
}

pub fn exit(args: Vec<Expression>) {
    match args[..] {
        [Expression::Int(exit_code)] => process::exit(exit_code as i32),
        [] => process::exit(0),
        _ => print_err_ln!("Warning: Ignoring malformed command."),
    }
}
