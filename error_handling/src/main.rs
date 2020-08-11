use std::fs::File;
use std::io::ErrorKind;

//fn main() {
    //let f = match File::open("hello.txt") {
        //Ok(file) => file,
        //Err(error) => match error.kind() {
            //ErrorKind::NotFound => match File::create("hello.txt") {
                //Ok(fc) => fc,
                //Err(e) => panic!("Problem creating the file: {:?}", e),
            //},
            //other_error => {
                //panic!("Problem opening the file: {:?}", other_error);
            //}
        //},
    //};
    //println!("unwrapped file: {:#?}", f);
//}

// more seasoned Rustacean would probably write the former in this manner
// removal of nested match expressions
fn main() {
    let _f = File::open("hello.text").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

// shortcut for panic on error and unwrap when not
fn main() {
    let f = File::open("hello.txt").unwrap();
}

// we can choose the panic error
fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

// --- CREATING CUSTOM TIMES FOR VALIDATION ---

// not ideal
fn not_ideal() {
    loop {
        // --snip--

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("the secret number will be between 1 and 100");
            continue;
        }

        match guess.cmp(&secret_number) {
            // --snip--
        }
    }
}

// more ideal
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }

    // getter
    pub fn value(&self) -> i32 {
        self.value
    }
}
