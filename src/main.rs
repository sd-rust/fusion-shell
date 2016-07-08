// Copyright (C) 2016  Sandeep Datta

#![feature(plugin)]
#![feature(slice_patterns)]

#![plugin(peg_syntax_ext)]


use std::io;
use std::io::prelude::*;

use fsh_parser::program;

#[macro_use]
mod utils;
mod asg;
mod commands;

use asg::*;

peg_file! fsh_parser("fsh.peg");

fn run_prog(prog: Vec<Expression>) {
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


fn do_repl() {
    utils::show_prompt();
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut buff = String::new();
    loop {
        if let Err(err) = stdin.read_line(&mut buff) {
            println!("Error: Could not read line from input stream. {}.", err);
            break;
        }
        {
            let line = buff.trim_right_matches('\n');

            // println!("{:?}", program(&line));
            let maybe_prog = program(line);
            match maybe_prog {
                Ok(prog) => run_prog(prog),
                Err(err) => print_err_ln!("Error: {}", err),
            }

            utils::show_prompt();
        }
        buff.clear();
    }
}

fn main() {
    utils::show_banner();
    do_repl();
}
