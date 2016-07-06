#![feature(plugin)]
#![feature(slice_patterns)]

#![plugin(peg_syntax_ext)]


use std::io;
use std::io::prelude::*;
use std::process;
use fsh_parser::program;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Expression {
    Int(i64),
    Str(String),
    CommandApp(String, Option<Vec<Expression>>) //Name, Args
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

fn exit(maybe_args: Option<Vec<Expression>>) {
    match maybe_args {
        Some(args) => {
            match &args[..] {
                &[Expression::Int(exit_code)] => process::exit(exit_code as i32),
                _ => println!("Ignoring malformed exit command.")
            }
        },
        None => {
            process::exit(0);
        }
    }
}

fn run_prog(prog: Vec<Expression>) {
    for expr in prog {
        match expr {
            Expression::Int(i) => println!("{}", i),
            Expression::Str(s) => println!("{}", s),
            Expression::CommandApp(name, maybe_args) => {
                match name.as_str() {
                    "exit" => exit(maybe_args),
                    _ => println!("Ignoring unknown command: {}", name)
                }
            }
        }
    }
}

fn do_repl() {
    let prompt = "fsh > ";
    print!("{}", prompt);
    io::stdout().flush().unwrap();
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
            
            print!("{}", prompt);
            io::stdout().flush().unwrap();
        }
        buff.clear();
    }
}

fn main() {
    println!("{}", BANNER);
    do_repl();
}
