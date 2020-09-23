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

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let root = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&root);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&root),
            Rc::weak_count(&root),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
    println!("### inner scope brace ends");

    println!("leaf parent = {:#?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    println!("### the leaf's parent (root) has been freed, leaving just the leaf by itself as a single strong ref");
}
