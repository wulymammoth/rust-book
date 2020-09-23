use std::{cell::RefCell, rc::{Rc, Weak}};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>, // Weak is a non-owning ref-counter
    children: RefCell<Vec<Rc<Node>>>,
}

pub fn main() {
    println!("\n--- n-ary tree with smart pointers (refcell, rc, weak) ---");

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let root = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&root);

    println!("leaf parent = {:#?}", leaf.parent.borrow().upgrade());
}
