// Copyright (C) 2016  Sandeep Datta

use commands;
use asg::Expression;

//For output formatting
use std::path::PathBuf;

fn print_path(val: commands::Result<PathBuf>) {
    match val {
        Ok(p) => println!("{}", p.display()),
        //TODO: Use the Display trait to print err
        Err(err) => print_err_ln!("Error: {:?}", err),
    }
}

pub fn run_prog(prog: Vec<Expression>) {
    for expr in prog {
        match expr {
            Expression::Int(i) => println!("{}", i),
            Expression::Str(s) => println!("{}", s),
            Expression::CommandApp(name, args) => {
                match name.as_str() {
                    "pwd" => print_path(commands::pwd(args)),
                    "cd" => commands::cd(args),
                    "exit" => commands::exit(args),
                    _ => print_err_ln!("Warning: Ignoring unknown command: {}", name),
                }
            }
        }
    }
}
