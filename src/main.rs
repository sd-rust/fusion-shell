#![feature(plugin)]
#![plugin(peg_syntax_ext)]

use std::io;
use std::io::prelude::*;
use arithmetic::expression;

peg_file! arithmetic("fsh.peg");

fn do_repl() {
    let prompt = "expression > ";
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut buff = String::new();
    loop {
        let _ = stdin.read_line(&mut buff).unwrap();
        {
            let line = buff.trim_right_matches("\n");

            if line == "exit" {
                break;
            }

            println!("{:?}", expression(&line));
            print!("{}", prompt);
            io::stdout().flush().unwrap();
        }
        buff.clear();
    }
}

fn main() {
    do_repl();
}
