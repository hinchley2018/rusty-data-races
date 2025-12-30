// Mutex that protects the data vector, and then we spawn three threads
// that each acquire a lock on the mutex and modify an element of the vector.
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    let handles: Vec<_> = (0..3)
        .map(|i| {
            // clone the Arc, not the Mutex
            let data = Arc::clone(&data); 
            thread::spawn(move || {
                println!("Thread {} trying to lock...", i);
                let mut data = data.lock().unwrap();
                println!("Thread {} acquired lock", i);
                thread::sleep(Duration::from_millis(100));
                data[i] += 1;
                println!("Thread {} released lock", i);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final data: {:?}", data.lock().unwrap());
}