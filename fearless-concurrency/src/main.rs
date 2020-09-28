use std::{thread, time::Duration};

mod closures_with_threads;
mod message_passing;
mod multiple_producers_via_transmitter_clone;
mod shared_state_concurrency;

fn main() {
    let handle = thread::spawn(|| { // returns a `JoinHandle`
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // if placed here, the spanwed thread handle will block the main thread from execution and exit
    // `handle.join().unwrap();`

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // use of `JoinHandle` to allow the spwaned thread to finish execution
    // - this is put after the main thread's for-loop
    // - results in the handle blocking the main thread from performing more work or exiting
    handle.join().unwrap();

    closures_with_threads::main();
    message_passing::main();
    multiple_producers_via_transmitter_clone::main();
    shared_state_concurrency::main();
}
