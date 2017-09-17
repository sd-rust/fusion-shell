// Copyright (C) 2016+ Sandeep Datta

use std::env;

// Font: Big. http://patorjk.com/software/taag/#p=display&f=Big&t=Fusion%20Shell
static BANNER: &'static str = r#"
 ______         _                _____ _          _ _ 
|  ____|       (_)              / ____| |        | | |
| |__ _   _ ___ _  ___  _ __   | (___ | |__   ___| | |
|  __| | | / __| |/ _ \|  _ \   \___ \|  _ \ / _ \ | |
| |  | |_| \__ \ | (_) | | | |  ____) | | | |  __/ | |
|_|   \__ _|___/_|\___/|_| |_| |_____/|_| |_|\___|_|_|
"#;

pub fn show_banner() {
    eprintln!("{}", BANNER);
}

pub fn prompt() -> String {
    match env::current_dir() {
        Ok(p) => format!("fsh {}> ", p.display()),
        Err(err) => {
            eprintln!("ERROR: Could not query current directory. {}.", err);
            "fsh > ".to_string()
        }
    }
}
