mod generic_data_types;
mod lifetimes;
mod traits;

fn main() {
    println!("\n");

    println!("--- 10.1 generic data types ---\n");
    generic_data_types::gen_data_types();

    println!("\n");

    println!("--- 10.2 traits: defining shared behavior ---\n");
    traits::shared_behavior();

    println!("--- 10.3 validating references with lifetimes\n");
    lifetimes::lifetimes();
}
