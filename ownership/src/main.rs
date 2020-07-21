fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // results in 5 (where the space is)
    println!("first word: {}", word);
    s.clear(); // this empties the String. making it ""

    // word still has the value of 5 here but is only relevant in the context of our original
    // string
    println!("cleared word {}", s);
}

// return index of the end of the word
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
