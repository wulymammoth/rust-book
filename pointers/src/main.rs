/// References
///
/// - they're pointers that ONLY borrow data
fn references() {
    println!("--- REFERENCES ---");
    let foo: &str = "baz";
    println!("printing a reference {}\n", &foo);
}

/// Smart Pointers
///
/// data structures that not only act like a pointer but also have
/// - additional metadata
/// - additional capabilities
/// - not unique to Rust as C++ has them as well
/// - allows us to own some memory and manipulate it
/// - typically implemented using structs
/// - they implement `Deref` and `Drop` traits
///
/// examples (most common):
/// - `String`
/// - `Vec<T>`
/// - `Box<T>`
/// - `Rc<T>`
/// - `Ref<T>`
/// - `RefMut<T>` and accessed through `RefCell<T>`

#[allow(dead_code)]
fn smart_pointers() {}

/// Box Pointers
///
/// - store data on the heap (instead of stack)
///   - stack contains a pointer to the heap data
/// - no performance overhead
/// - use cases:
///   - type whose size is unknown at compile time and want to use the value of the type in context
///   that requires an exact size (e.g., recursive types)
///   - large amount of data and want to transfer ownership but disallow copying (mitigating stack
///   allocation and copying that is slow)
///   - 3. trait objects -- desire to own a value and we only care about type that implements a particular trait
fn box_pointers() {
    println!("--- BOX POINTERS ---");
    let b = Box::new(5);
    println!("b = {}\n", b);

    // NOTE: recursive types with boxes
    // cons lists - data type that is common in functional programming languages
    enum List {
        Cons(i32, Box<List>), // Box is a "smart" pointer (fixed size)
        Nil,
    }

    use List::{Cons, Nil};

    #[allow(unused_variables)]
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

/// dereference pointer (*) allows us to follow the pointer to the data
fn dereference() {
    println!("--- DEREF TRAIT ---");
    let x = 5;
    let y = MyBox::new(x); // setting `y` to a reference of `x`

    assert_eq!(5, x);
    // because y is a reference to a reference, we need to follow it to obtain the value
    assert_eq!(5, *y); // * is a call to the `deref` method
}

/// defining our own smart pointer
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T; // defines associated type for `Deref` trait to use

    fn deref(&self) -> &T {
        &self.0 // this avoids moving the value out of self by returning a reference
    }
}

/// deref coercion
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    references();
    box_pointers();
    dereference();

    // deref coercion with MyBox
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // deref coercion saves us from having to write...
    // hello(&(*m)[..]);
    // - the (*m) does the deref into MyBox<String> and into String
    // - the & and [..] take a string slice of the String that is equal to the entire string to
    // match the signature of hello
}
