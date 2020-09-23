/// reference cycles can leak memory
///
/// LINK: https://doc.rust-lang.org/book/ch15-06-reference-cycles.html
///
/// - this happens when we accidentally create memory that is never cleaned up
/// - NOT one of Rust's guarantees and can rely on Rust to catch them
/// - preventing data races is

// creating a reference cycle
// - where the ref count never reaches 0
use List::{Cons, Nil};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

pub fn main() {
    println!("\n--- ref cycles can cause memo leaks ---");

    // creating a ref-counted list containing 5
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    // creating another ref-counted list containing 10
    // and point the list to a
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", Rc::clone(&b));

    // HERE we modify a such that it points back to `b` instead of `Nil`
    // completing our cycle
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}
