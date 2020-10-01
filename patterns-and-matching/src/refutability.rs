pub fn main() {
    println!("\n=== 18.2 patterns and matching | refutability ===\n");
    refutable_pattern();
}

fn refutable_pattern() {
    let some_option_value: Option<u8> = Some(5);

    // pattern `None` is not covered, so instead of...
    // let Some(x) = some_option_value;

    // use `if let`
    if let Some(x) = some_option_value {
        println!("{}", x);
    }
}
