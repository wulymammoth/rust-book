pub fn main() {
    println!("\n-- 19.3 advanced types\n");
    newtype_for_type_safety_and_abstraction();
    the_never_type_that_never_returns();
    dynamically_sized_types_and_the_sized_trait();
}

/// NOTE: dynamically sized types AKA DSTs or "unsized types"
fn dynamically_sized_types_and_the_sized_trait() {
    println!("19.3 : Rust needs to know how much memory to allocate for any value of a particular type");
    println!("19.3 : GOLDEN RULE of DSTs - we must always put values of dynamically sized types behind a pointer of some kind\n");
    println!("19.3 : we can combine `str` with all sorts of pointers...");

    println!("\n19.3 : by default, generic functions will work only on types that have a known size at compile time, but we can relax this restriction with the following...\n");
    // a trait bound on `?Sized` is the opposite of a trait bound on `Sized`
    // - read as: "T may or may not be Sized" (maybe)
    #[allow(dead_code)]
    fn generic<T: ?Sized>(_t: &T) { // we've chosen a reference here because the type may not be sized
        // --snip--
    }
}

/// NOTE: think encapsulation here (public vs private API)
fn newtype_for_type_safety_and_abstraction() {
    println!("19.3 : type aliases\n");
    // Rust also provides a way to create "type aliases"
    // - lose benefit of type-checking using the newtype pattern
    // - primary benefit is to reduce repitition
    type Kilometers = i32; // synonym for i32
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}\n", x + y);

    // reducing repitition
    //Box<dyn Fn() + Send + 'static'>
    // BAD
    //let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

    //fn take_long_type(f: Box<dyn Fn() + Send + 'static>) {}

    //fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
    //// --snip--
    //}

    // GOOD
    // - a "thunk" is a word for code to be evaluated at a later time
    type Thunk = Box<dyn Fn() + Send + 'static>;
    let _f: Thunk = Box::new(|| println!("hi"));
    #[allow(dead_code)]
    fn takes_long_type(_f: Thunk) {}
    //fn returns_long_type() -> Thunk {}

    // NOTE: type aliases are commonly used with the Result<T, E> type for reducing repition
    // - example of functions in the Write trait
    use std::fmt;
    use std::io::Result;

    // type Result<T> = std::result::Result<T, std::io::Error>;
    // ^ this declaration is in the std::io module, so we can simplify
    // -- before
    //pub trait Write {
    //fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    //fn flush(&mut self) -> Result<(), Error>;

    //fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    //fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
    //}

    // -- after
    pub trait WriteV2 {
        fn write(&mut self, buf: &[u8]) -> Result<usize>;
        fn flush(&mut self) -> Result<()>;

        fn write_all(&mut self, buf: &[u8]) -> Result<()>;
        fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
    }
}

/// NOTE: `!` (never) type is known as the "empty type" in type theory
/// because it has no values
fn the_never_type_that_never_returns() {
    println!("19.3 : a function that returns never\n");
    // fn bar() -> ! { // "the function `bar` returns never"
    // --snip--
    // }

    println!("19.3 : `continue` has a `!` (never value)\n");
    //let guess: u32 = match guess.trim().parse() {
    //Ok(num) => num,
    //Err(_) => continue,
    //}

    println!("19.3 : never type is useful with the `panic!` macro\n");
    //impl<T> Option<T> {
    //pub fn unwrap(self) -> T {
    //match self {
    //Some(val) => val,
    //None => panic!("called `Option::unwrap()` on a `None` value");
    //}
    //}
    //}

    println!("19.3 : are expressions that have a `!` (never type)\n");
    print!("forever ");
    let mut times: u8 = 10;
    loop {
        if times == 0 {
            break;
        }
        println!("and ever ");
        times -= 1;
    }
}
