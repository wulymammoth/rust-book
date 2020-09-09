use std::{env, process};

use minigrep::Config;

fn main() {
    // we have to annotate `collect`, because Rust doesn't know what sort of a collection we want
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("\n--- {} ---\n", config.program_name);
    println!("Searching for: '{}'", config.query);
    println!("In file: {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
