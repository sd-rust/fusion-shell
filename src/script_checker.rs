// Copyright (C) 2017+ Sandeep Datta
use fsh_parser;
use std::fs::File;
use std::io::prelude::*;

pub fn parse_script(path: &str) {
    let mut file = File::open(path).unwrap();
    let mut script_text = String::new();
    file.read_to_string(&mut script_text).unwrap();
    match fsh_parser::program(&script_text) {
        Ok(prog) => {
            println!("{:#?}", prog);
        }
        Err(err) => println_err!("Error: {}", err),
    }
}
