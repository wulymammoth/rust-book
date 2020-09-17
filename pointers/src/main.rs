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

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

fn main() {
    references();
    box_pointers();
}
