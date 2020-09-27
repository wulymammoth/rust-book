use std::thread;

pub fn main() {
    println!("\n--- fearless concurrency : closures with threads ---\n");

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
