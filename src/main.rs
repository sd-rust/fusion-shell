//Copyright (C) 2016  Sandeep Datta

#![feature(plugin)]
#![feature(slice_patterns)]

#![plugin(peg_syntax_ext)]


use std::io;
use std::io::prelude::*;
use std::process;
use std::env;
use std::path::Path;
use fsh_parser::program;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Expression {
    Int(i64),
    Str(String),
    CommandApp(String, Vec<Expression>) //Name, Args
}

peg_file! fsh_parser("fsh.peg");

//Font: Big. http://patorjk.com/software/taag/#p=display&f=Big&t=Fusion
static BANNER:&'static str = r#"
 ______         _                _____ _          _ _ 
|  ____|       (_)              / ____| |        | | |
| |__ _   _ ___ _  ___  _ __   | (___ | |__   ___| | |
|  __| | | / __| |/ _ \|  _ \   \___ \|  _ \ / _ \ | |
| |  | |_| \__ \ | (_) | | | |  ____) | | | |  __/ | |
|_|   \__ _|___/_|\___/|_| |_| |_____/|_| |_|\___|_|_|
"#;

//MFU commands: history | cut -d" " -f1 | sort | uniq -c | sort -r -n -k 1 | head -50

fn pwd(args: Vec<Expression>) {
    match &args[..] {
        &[] => {
            let p = env::current_dir().unwrap();
            println!("{}", p.display());
        },
        _ => println!("Ignoring malformed command.")
    }
}

fn cd(args: Vec<Expression>) {
    match &args[..] {
        &[Expression::Str(ref s)] => {
            let root = Path::new(s);
            if !env::set_current_dir(&root).is_ok() {
                println!("Error: could not set current directory.");
            }
        },
        &[] => {
            let p = env::current_dir().unwrap();
            println!("{}", p.display());
        },
        _ => println!("Ignoring malformed command.")
    }
}

// fn grep(_args: Vec<Expression>) {
//     println!("Not yet implemented!");
// }

// fn find(_args: Vec<Expression>) {
//     println!("Not yet implemented!");
// }

// fn ls(_args: Vec<Expression>) {
//     println!("Not yet implemented!");
// }

// fn echo(_args: Vec<Expression>) {
//     println!("Not yet implemented!");
// }

fn exit(args: Vec<Expression>) {
    match &args[..] {
        &[Expression::Int(exit_code)] => process::exit(exit_code as i32),
        &[] => process::exit(0),
        _ => println!("Ignoring malformed command.")
    }
}

fn run_prog(prog: Vec<Expression>) {
    for expr in prog {
        match expr {
            Expression::Int(i) => println!("{}", i),
            Expression::Str(s) => println!("{}", s),
            Expression::CommandApp(name, args) => {
                match name.as_str() {
                    "pwd"   => pwd(args),
                    "cd"    => cd(args),
                    "exit"  => exit(args),
                    _ => println!("Ignoring unknown command: {}", name)
                }
            }
        }
    }
}

fn prompt() -> String {
    let p = env::current_dir().unwrap();
    format!("fsh {}> ", p.display())
}

fn show_prompt() {
    print!("{}", prompt());
    io::stdout().flush().unwrap();
}

fn do_repl() {
    show_prompt();
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut buff = String::new();
    loop {
        let _ = stdin.read_line(&mut buff).unwrap();
        {
            let line = buff.trim_right_matches("\n");

            //println!("{:?}", program(&line));
            let maybe_prog = program(&line);
            match maybe_prog {
                Ok(prog) => run_prog(prog),
                Err(err) => println!("Error: {:?}", err),
            }
            
            show_prompt();
        }
        buff.clear();
    }
}

fn main() {
    println!("{}", BANNER);
    do_repl();
}
