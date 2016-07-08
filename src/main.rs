// Copyright (C) 2016  Sandeep Datta

#![feature(plugin)]
#![feature(slice_patterns)]

#![plugin(peg_syntax_ext)]

#[macro_use]
mod utils;
mod asg;
mod commands;
mod interpreter;
mod repl;

peg_file! fsh_parser("fsh.peg");

fn main() {
    utils::show_banner();
    repl::do_repl();
}
