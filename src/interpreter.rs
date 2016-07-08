// Copyright (C) 2016  Sandeep Datta

use commands::{self, PipeValue, PipeError};
use asg::{Expression, CommandApplication};

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
    match expr {
        Expression::Int(val) => Ok(PipeValue::Int(val)),
        Expression::Str(val) => Ok(PipeValue::Str(val)),
        Expression::Command(CommandApplication {name, args}) => {
            match name.as_str() {
                "pwd" => commands::pwd(args),
                "cd" => commands::cd(args),
                "exit" => commands::exit(args),
                _ => Err(PipeError::BadCommand),
            }
        }
        Expression::Map(_, _) => unimplemented!(),
    }
}

pub fn run_prog(prog: Vec<Expression>) {
    for expr in prog {
        print(run_expr(expr));
        println!("");
    }
}
