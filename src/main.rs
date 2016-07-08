// Copyright (C) 2016  Sandeep Datta

#![feature(plugin)]
#![feature(slice_patterns)]

#![plugin(peg_syntax_ext)]


use std::io;
use std::io::prelude::*;
use std::process;
use std::env;
use std::path::Path;
use fsh_parser::program;

macro_rules! print_err_ln {
    ($($arg:tt)*) => ({
        use std::io::Write;
        match writeln!(&mut ::std::io::stderr(), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr (file handle closed?): {}", x),
        }
    })
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Expression {
    Int(i64),
    Str(String),
    CommandApp(String, Vec<Expression>), // Name, Args
}

peg_file! fsh_parser("fsh.peg");

// Font: Big. http://patorjk.com/software/taag/#p=display&f=Big&t=Fusion
static BANNER: &'static str = r#"
 ______         _                _____ _          _ _ 
|  ____|       (_)              / ____| |        | | |
| |__ _   _ ___ _  ___  _ __   | (___ | |__   ___| | |
|  __| | | / __| |/ _ \|  _ \   \___ \|  _ \ / _ \ | |
| |  | |_| \__ \ | (_) | | | |  ____) | | | |  __/ | |
|_|   \__ _|___/_|\___/|_| |_| |_____/|_| |_|\___|_|_|
"#;

fn print_curdir() {
    match env::current_dir() {
        Ok(p) => println!("{}", p.display()),
        Err(err) => {
            print_err_ln!("Error: Could not query current directory. {}.", err);
        }
    }
}

fn pwd(args: Vec<Expression>) {
    match args[..] {
        [] => print_curdir(),
        _ => print_err_ln!("Warning: Ignoring malformed command."),
    }
}

fn cd(args: Vec<Expression>) {
    match args[..] {
        [Expression::Str(ref s)] => {
            let root = Path::new(s);
            if let Err(err) = env::set_current_dir(&root) {
                print_err_ln!("Error: Could not set current directory. {}.", err);
            }
        }
        [] => print_curdir(),
        _ => print_err_ln!("Warning: Ignoring malformed command."),
    }
}

fn exit(args: Vec<Expression>) {
    match args[..] {
        [Expression::Int(exit_code)] => process::exit(exit_code as i32),
        [] => process::exit(0),
        _ => print_err_ln!("Warning: Ignoring malformed command."),
    }
}

fn run_prog(prog: Vec<Expression>) {
    for expr in prog {
        match expr {
            Expression::Int(i) => println!("{}", i),
            Expression::Str(s) => println!("{}", s),
            Expression::CommandApp(name, args) => {
                match name.as_str() {
                    "pwd" => pwd(args),
                    "cd" => cd(args),
                    "exit" => exit(args),
                    _ => print_err_ln!("Warning: Ignoring unknown command: {}", name),
                }
            }
        }
    }
}

fn prompt() -> String {
    match env::current_dir() {
        Ok(p) => format!("fsh {}> ", p.display()),
        Err(err) => {
            print_err_ln!("Error: Could not query current directory. {}.", err);
            "fsh > ".to_string()
        }
    }
}

fn show_prompt() {
    print!("{}", prompt());
    io::stdout()
        .flush()
        .unwrap_or_else(|err| print_err_ln!("Warning: could not flush output stream. {}.", err));
}

fn do_repl() {
    show_prompt();
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

            show_prompt();
        }
        buff.clear();
    }
}

fn main() {
    print_err_ln!("{}", BANNER);
    do_repl();
}
