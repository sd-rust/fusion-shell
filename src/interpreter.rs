// Copyright (C) 2016  Sandeep Datta

use commands::{self, PipeError};
use asg::{Expression, CommandApplication, PipeValue};

//For output formatting
fn print(val: commands::Result<PipeValue>) {
    match val {
        Ok(pv) => {
            match pv {
                PipeValue::None => (),
                PipeValue::Int(val) => print!("{}", val),
                PipeValue::Str(val) => print!("{}", val),
                PipeValue::Path(val) => print!("{}", val.display()),
            }
        }
        Err(err) => print_err_ln!("Error: {}", err),
    }
}

fn run_expr(expr: Expression) -> commands::Result<PipeValue>{
    //println!("expr: {:?}", expr);

    match expr {
        Expression::Value(pipe_val) => Ok(pipe_val),
        Expression::Command(CommandApplication {name, args}) => {
            
            //println!("args: {:?}", args);

            let mut computed_args: Vec<PipeValue> = vec![];
                
            for arg in args {
                computed_args.push(try!(run_expr(arg)));
            }

            //println!("computed_args: {:?}", computed_args);

            match name.as_str() {
                "pwd" => commands::pwd(computed_args),
                "cd" => commands::cd(computed_args),
                "exit" => commands::exit(computed_args),
                _ => Err(PipeError::BadCommand),
            }
        }
    }
}

pub fn run_prog(prog: Vec<Expression>) {
    for expr in prog {
        print(run_expr(expr));
        println!("");
    }
}
