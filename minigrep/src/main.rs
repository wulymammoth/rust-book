use std::{env, fs};

fn main() {
    // we have to annotate, because Rust doesn't know what sort of a collection we want
    let args: Vec<String> = env::args().collect();

    let program_name = &args[0];
    let query = &args[1];
    let filename = &args[2];

    println!("\n--- {} ---\n", program_name);
    println!("Searching for: '{}'", query);
    println!("In file: {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
