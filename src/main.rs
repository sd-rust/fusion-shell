#![feature(plugin)]
#![plugin(peg_syntax_ext)]

use std::io;
use std::io::prelude::*;
use arithmetic::expression;

peg_file! arithmetic("fsh.peg");

//Font: Big. http://patorjk.com/software/taag/#p=display&f=Big&t=Fusion
static BANNER:&'static str = r#"
 ______         _                _____ _          _ _ 
|  ____|       (_)              / ____| |        | | |
| |__ _   _ ___ _  ___  _ __   | (___ | |__   ___| | |
|  __| | | / __| |/ _ \|  _ \   \___ \|  _ \ / _ \ | |
| |  | |_| \__ \ | (_) | | | |  ____) | | | |  __/ | |
|_|   \__ _|___/_|\___/|_| |_| |_____/|_| |_|\___|_|_|
"#;

fn do_repl() {
    let prompt = "expression > ";
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut buff = String::new();
    loop {
        let _ = stdin.read_line(&mut buff).unwrap();
        {
            let line = buff.trim_right_matches("\n");

            if line == "exit" {
                break;
            }

            println!("{:?}", expression(&line));
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
