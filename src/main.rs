// Copyright (C) 2016+ Sandeep Datta

#![feature(plugin)]
#![feature(slice_patterns)]

extern crate rustyline;

#[macro_use]
mod utils;
mod asg;
mod commands;
mod interpreter;
mod repl;
mod script_checker;

mod fsh_parser {
    include!(concat!(env!("OUT_DIR"), "/fsh.rs"));
}

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        1 => {
            utils::show_banner();
            repl::do_repl();
        },
        _ => {
            println_err!("Running:{}", args[1]);
            script_checker::parse_script(&args[1]);
        }
    }
    
}
