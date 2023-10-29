use std::{env, process};

use minigrep;

fn main() {
    let config = minigrep::Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("There was a problem parsing the arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
