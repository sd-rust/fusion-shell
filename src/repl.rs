// Copyright (C) 2016+ Sandeep Datta

// use std::process;

use utils;
// use interpreter::{self, Exit};

use fsh_parser;

use rustyline::error::ReadlineError;
use rustyline::Editor;

pub fn do_repl() {
    let mut rl = Editor::<()>::new();
    let history_file = "fsh_history";
    if let Err(_) = rl.load_history(history_file) {
        eprintln!("No previous history.");
    }

    loop {
        let readline = rl.readline(&utils::prompt());
        match readline {
            Ok(line) => {
                let maybe_prog = fsh_parser::program(&line);

                match maybe_prog {
                    Ok(prog) => {
                        // Only add syntactically valid entries
                        rl.add_history_entry(&line);

                        // TODO: Return exit code from do_repl() and call
                        // process::exit() from main()
                        // if let Exit::Yes(ecode) = interpreter::run_prog(prog) {
                        //     rl.save_history(history_file).unwrap();
                        //     process::exit(ecode);
                        // }

                        println!("{:#?}", prog);
                    }
                    Err(err) => eprintln!("Error: {}", err),
                }
            }
            Err(ReadlineError::Interrupted) => {
                eprintln!("Ctrl-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                eprintln!("Ctrl-D");
                break;
            }
            Err(err) => {
                eprintln!("Error: {}", err);
                break;
            }
        }
    }
    rl.save_history(history_file).unwrap();
}
