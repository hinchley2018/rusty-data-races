use std::sync::{Arc, Mutex};
use std::thread;

/// A simple bank account simulation that demonstrates safe shared mutation
/// using `Arc<Mutex<i32>>`. Each thread will lock the mutex, read the
/// current balance, subtract 50, write the new balance, and print what it did.
pub fn run_bank_example() {
    let balance = Arc::new(Mutex::new(100));

    let handles: Vec<_> = (0..2)
        .map(|thread_id| {
            // clone the Arc, not the Mutex
            let bal = Arc::clone(&balance);
            thread::spawn(move || {
                // Acquire the lock, modify the balance, then release when scope ends
                let mut guard = bal.lock().unwrap();
                let old_balance = *guard;
                let new_balance = old_balance - 50;
                *guard = new_balance;
                println!(
                    "Thread {} updated balance: {} -> {}",
                    thread_id, old_balance, new_balance
                );
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    // Print final balance by locking the mutex and reading the value
    println!("Final balance: {}", *balance.lock().unwrap());
}
