use std::{sync::mpsc, thread, time::Duration}; // multiple producer, single consumer

pub fn main() {
    println!("\n--- 16.2 fearless concurrency : creating multiple producer by cloning the transmitter ---\n");

    let (tx, rx) = mpsc::channel();

    let cloned_trasmitter = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("clone: hi"),
            String::from("clone: from"),
            String::from("clone: the"),
            String::from("clone: thread"),
        ];

        for val in vals {
            cloned_trasmitter.send(val).unwrap();
            thread::sleep(Duration::from_millis(750));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("message"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
