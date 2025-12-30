use std::sync::{Arc, Mutex};
use std::thread;

/// A transaction for the account.
#[derive(Debug)]
pub enum Transaction {
    Deposit(i32),
    Withdraw(i32),
}

/// Process a list of transactions across `thread_count` threads.
///
/// Returns a tuple containing the final balance and a list of transaction
/// indexes that failed (withdrawals that would have caused an overdraft).
pub fn process_transactions(
    initial_balance: i32,
    transactions: Vec<Transaction>,
    thread_count: usize,
) -> (i32, Vec<usize>) {
    let threads = thread_count.max(1);
    let balance = Arc::new(Mutex::new(initial_balance));

    // Attach an index to each transaction so callers can identify failures.
    let indexed: Vec<(usize, Transaction)> = transactions.into_iter().enumerate().collect();

    // Distribute transactions round-robin across worker buckets.
    let mut buckets: Vec<Vec<(usize, Transaction)>> = (0..threads).map(|_| Vec::new()).collect();
    for (i, tx) in indexed {
        buckets[i % threads].push((i, tx));
    }

    let handles: Vec<_> = buckets
        .into_iter()
        .map(|bucket| {
            let bal = Arc::clone(&balance);
            thread::spawn(move || {
                let mut failed = Vec::new();
                for (idx, tx) in bucket {
                    match tx {
                        Transaction::Deposit(a) => {
                            let mut g = bal.lock().unwrap();
                            *g += a;
                        }
                        Transaction::Withdraw(a) => {
                            let mut g = bal.lock().unwrap();
                            if *g >= a {
                                *g -= a;
                            } else {
                                // record failure index
                                failed.push(idx);
                            }
                        }
                    }
                }
                failed
            })
        })
        .collect();

    // Gather failed transaction indices from all threads
    let mut failed: Vec<usize> = Vec::new();
    for h in handles {
        let mut part = h.join().unwrap();
        failed.append(&mut part);
    }

    let final_balance = *balance.lock().unwrap();
    (final_balance, failed)
}

// Unit tests moved to `tests/integration_tests.rs` so they can exercise the
// public API as integration tests. Keep library code focused on functionality.
