use std::sync::{Arc, Mutex};
use std::thread;

fn shared_mutex() {
    println!("\n--- 16.3 fearless concurrency : atomic ref counting (thread safety)\n");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

pub fn main() {
    println!("\n--- 16.3 fearless concurrency : shared-stated concurrency ---\n");

    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    // sharing a `Mutex<T>` between multiple threads
    shared_mutex();
}
