// Copyright (C) 2016  Sandeep Datta

use commands::{self, StreamError, Result};
use asg::{Expression, CommandApplication, StreamElement};

pub enum Exit {
    No,
    Yes(i32),
}

// TODO: Rename print to handle_result or something better
// For output formatting
fn print(val: Result<StreamElement>) -> Exit {
    match val {
        Ok(pv) => {
            match pv {
                StreamElement::None => (),
                StreamElement::Int(val) => print!("{}", val),
                StreamElement::Str(val) => print!("{}", val),
                StreamElement::Path(val) => print!("{}", val.display()),
                StreamElement::Exit(ecode) => return Exit::Yes(ecode),
            }
        }
        Err(err) => print_err!("Error: {}", err),
    }

    Exit::No
}

fn run_expr(expr: Expression) -> Result<StreamElement> {
    // println!("expr: {:?}", expr);

    match expr {
        Expression::Value(pipe_val) => Ok(pipe_val),
        Expression::Command(CommandApplication { name, args }) => {

            // println!("args: {:?}", args);

            let mut computed_args: Vec<StreamElement> = vec![];

            for arg in args {
                computed_args.push(try!(run_expr(arg)));
            }

            // println!("computed_args: {:?}", computed_args);

            match name.as_str() {
                "pwd" => commands::pwd(computed_args),
                "cd" => commands::cd(computed_args),
                "exit" => commands::exit(computed_args),
                _ => Err(StreamError::BadCommand),
            }
        }
    }
}

pub fn run_prog(prog: Vec<Expression>) -> Exit {
    for expr in prog {
        let maybe_exit = print(run_expr(expr));
        println!("");
        if let Exit::Yes(_) = maybe_exit {
            return maybe_exit;
        }
    }

    Exit::No
}
