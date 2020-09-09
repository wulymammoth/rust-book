use std::{error::Error, fs};

// Box<dyn Error> is a trait object (`dyn` is short for "dynamic")
// - simply means that function will return a type that implements the `Error` trait
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // rather than panicking, we return the error value here using the `?` operator
    let contents = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", contents);
    Ok(()) // idiomatic way of of specifying that we care only about the side-effects
}

pub struct Config {
    pub program_name: String,
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // error-handling
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // we are trading performance for simplicity here when we use clone
        // we are avoiding lifetime annotations
        let program_name = args[0].clone();
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {
            program_name,
            query,
            filename,
        })
    }
}
