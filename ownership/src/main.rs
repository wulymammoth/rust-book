fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // results in 5 (where the space is)
    s.clear(); // this empties the String. making it ""

    // word still has the value of 5 here but is only relevant in the context of our original
    // string
}

// return index of the end of the word
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    // enumerate is similar to Python's here that returns a tuple with index and a ref to the the
    // element
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // checking for space char
            return i;
        }
    }

    s.len()
}
