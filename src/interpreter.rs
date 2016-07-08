// Copyright (C) 2016  Sandeep Datta

use commands;
use asg::Expression;

pub fn run_prog(prog: Vec<Expression>) {
    for expr in prog {
        match expr {
            Expression::Int(i) => println!("{}", i),
            Expression::Str(s) => println!("{}", s),
            Expression::CommandApp(name, args) => {
                match name.as_str() {
                    "pwd" => commands::pwd(args),
                    "cd" => commands::cd(args),
                    "exit" => commands::exit(args),
                    _ => print_err_ln!("Warning: Ignoring unknown command: {}", name),
                }
            }
        }
    }
}
