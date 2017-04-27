// Copyright (C) 2016  Sandeep Datta

use commands::{self, StreamError, Stream};
use asg::{Expression, FunctionApplication, Primitive};

pub enum Exit {
    No,
    Yes(i32),
}

// TODO: Rename print to handle_result or something better
// For output formatting
fn print(stream: Stream) -> Exit {
    for maybe_val in stream {
        match maybe_val {
            Ok(val) => {
                match val {
                    Primitive::Exit(ecode) => return Exit::Yes(ecode),
                    _ => print!("{}", val),
                }
            }
            Err(err) => print_err!("Error: {}", err),
        }
    }

    Exit::No
}

fn run_expr(expr: Expression) -> Stream {
    // println!("expr: {:?}", expr);

    match expr {
        Expression::Value(pipe_val) => vec![Ok(pipe_val)],
        Expression::Function(FunctionApplication { name, args }) => {

            // println!("args: {:?}", args);

            let mut computed_args: Vec<Stream> = vec![];

            //Evaluate args
            for arg in args {
                computed_args.push(run_expr(arg));
            }

            // println!("computed_args: {:?}", computed_args);

            match name.as_str() {
                "pwd" => commands::pwd(computed_args),
                "cd" => commands::cd(computed_args),
                "exit" => commands::exit(computed_args),
                _ => vec![Err(StreamError::BadCommand)],
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
