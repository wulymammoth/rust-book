use std::{sync::mpsc, thread}; // multiple producer, single consumer

pub fn main() {
    println!("\n--- fearless concurrency : message passing ---\n");

    // transmitter (sender), receiver
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
