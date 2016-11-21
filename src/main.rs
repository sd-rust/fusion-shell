// Copyright (C) 2016  Sandeep Datta

#![feature(plugin)]
#![feature(slice_patterns)]

extern crate rustyline;

#[macro_use]
mod utils;
mod asg;
mod commands;
mod interpreter;
mod repl;

mod fsh_parser {
    include!(concat!(env!("OUT_DIR"), "/fsh.rs"));
}

fn main() {
    utils::show_banner();
    repl::do_repl();
}
