use std::{env, fs};

fn main() {
    // we have to annotate `collect`, because Rust doesn't know what sort of a collection we want
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("\n--- {} ---\n", config.program_name);
    println!("Searching for: '{}'", config.query);
    println!("In file: {}", config.filename);

    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

struct Config {
    program_name: String,
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        // we are trading performance for simplicity here when we use clone
        // we are avoiding lifetime annotations
        let program_name = args[0].clone();
        let query = args[1].clone();
        let filename = args[2].clone();

        Config {
            program_name,
            query,
            filename,
        }
    }
}
