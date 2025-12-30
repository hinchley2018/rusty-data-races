// Mutex that protects the data vector, and then we spawn three threads
// that each acquire a lock on the mutex and modify an element of the vector.
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
pub fn run_simple_safe_threads() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    // Spawn three threads
    let handles: Vec<_> = (0..3)
        .map(|thread_id| {
            // clone the Arc, not the Mutex
            let data = Arc::clone(&data);
            thread::spawn(move || {
                println!("Thread {} trying to lock...", thread_id);
                let mut data = data.lock().unwrap();
                println!("Thread {} acquired lock", thread_id);
                thread::sleep(Duration::from_millis(100));
                data[thread_id] += 1;
                println!("Thread {} released lock", thread_id);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final data: {:?}", data.lock().unwrap());
}
