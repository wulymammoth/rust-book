fn main() {
    delimit("creating a new vector (rare)");
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);

    delimit("creating a vector more common (with `vec!` macro)");
    let mut v = vec![1, 2, 3];
    println!("{:?}", v);

    for i in 4..10 {
        v.push(i);
    }

    delimit("reading elements in a vector");
    // 1. indexing syntax
    let third: &i32 = &v[2];
    println!("the third element is {} (using indexing)", third);
    // 2. get method (this gives us an `Option<&T>`)
    match v.get(2) {
        Some(third) => println!("the third element is {} (using get)", third),
        None => println!("there is no third element (using get method)"),
    }

    let v = vec![1, 2, 3, 4, 5, 6];
    let does_not_exist = v.get(100);
    println!(
        "attempting to access the 100th indexed value {:?}",
        does_not_exist
    );

    // results in panic
    //let does_not_exist = &v[100];
    //println!("attempting to access the 100th indexed value {}", does_not_exist);

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let first = v[0];
    v.push(6);
    println!("the first element is: {}", first);

    delimit("iterating over values in a a vector");
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    delimit("using an enum to store multiple types (like a named tuple w/ type checking)");
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("foobar")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("spreadsheet row: {:#?}", row);

    delimit("appending to a string");
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {}", s1);
    println!("s2 is {}", s2);

    delimit("appending a single character to a string");
    let mut s1 = String::from("foo");
    s1.push('l'); // single quote
    println!("{}", s1); // "fool"

    delimit("concatenation with the + operator or the format! macro");
    let s1 = String::from("Hello, ");
    let s2 = String::from("word!");
    let s3 = s1 + &s2;
    println!("concatenation using + (with s1 moved): {}", s3);

    delimit("string slicing (avoid indexing into string)");
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("s: {}", s); // Зд (because each of those chars is two bytes)
}

fn delimit(msg: &str) {
    println!("\n--- {} ---", msg);
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
