use std::{env, process};

use minigrep::Config;

fn main() {
    // instead of borrowing &args, we pass in env.args(), because it's an iterator
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("\n--- {} ---\n", config.program_name);
    println!("Searching for: '{}'", config.query);
    println!("In file: {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
