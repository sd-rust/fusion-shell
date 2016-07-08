// Copyright (C) 2016  Sandeep Datta

use std::io;
use std::io::BufRead;

use utils;
use interpreter;

use fsh_parser;

pub fn do_repl() {
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

            let maybe_prog = fsh_parser::program(line);

            match maybe_prog {
                Ok(prog) => interpreter::run_prog(prog),
                Err(err) => print_err_ln!("Error: {}", err),
            }

            utils::show_prompt();
        }
        buff.clear();
    }
}
