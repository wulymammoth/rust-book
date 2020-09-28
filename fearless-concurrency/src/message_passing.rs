use std::{sync::mpsc, thread, time::Duration}; // multiple producer, single consumer

#[test]
#[ignore = "safe concurrency"]
fn ownership_transference() {
    println!("\n--- 16.2 fearless concurrency : channels and ownership transference ---\n");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {},", received);
}

fn multiple_values_and_waiting_receiver() {
    println!("\n--- 16.2 fearless concurrency : sending multiple values and seeing the receiver wait ---\n");

    // spwanwed thread will now send multiple messages and pause for a second between each message
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            // this causes the receive to wait
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

pub fn main() {
    println!("\n--- 16. fearless concurrency : message passing ---\n");

    // transmitter (sender), receiver
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // channels and ownership transference
    //ownership_transference();

    // sending multiple values and seeing the receiver wait
    multiple_values_and_waiting_receiver();
}
