// Copyright (C) 2016  Sandeep Datta

use std::io;
use std::env;
use std::io::prelude::*;

// Font: Big. http://patorjk.com/software/taag/#p=display&f=Big&t=Fusion
static BANNER: &'static str = r#"
 ______         _                _____ _          _ _ 
|  ____|       (_)              / ____| |        | | |
| |__ _   _ ___ _  ___  _ __   | (___ | |__   ___| | |
|  __| | | / __| |/ _ \|  _ \   \___ \|  _ \ / _ \ | |
| |  | |_| \__ \ | (_) | | | |  ____) | | | |  __/ | |
|_|   \__ _|___/_|\___/|_| |_| |_____/|_| |_|\___|_|_|
"#;

macro_rules! print_err_ln {
    ($($arg:tt)*) => ({
        use std::io::Write;
        match writeln!(&mut ::std::io::stderr(), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr (file handle closed?): {}", x),
        }
    })
}

pub fn show_banner() {
    print_err_ln!("{}", BANNER);
}

pub fn show_prompt() {
    print!("{}", prompt());
    io::stdout()
        .flush()
        .unwrap_or_else(|err| print_err_ln!("Warning: could not flush output stream. {}.", err));
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
