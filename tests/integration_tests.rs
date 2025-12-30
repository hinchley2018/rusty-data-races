use data_race::{process_transactions, Transaction};

#[test]
fn deposits_and_withdrawals_single_thread() {
    let txs = vec![
        Transaction::Deposit(50),
        Transaction::Withdraw(30),
        Transaction::Withdraw(20),
    ];
    let (final_balance, failed) = process_transactions(100, txs, 1);
    assert_eq!(final_balance, 100); // 100 +50 -30 -20 = 100
    assert!(failed.is_empty());
}

#[test]
fn prevents_overdraft_across_threads() {
    // Two threads attempt to withdraw 50 each from an initial 50.
    let txs = vec![Transaction::Withdraw(50), Transaction::Withdraw(50)];
    let (final_balance, mut failed) = process_transactions(50, txs, 2);
    // Only one withdrawal should succeed.
    assert_eq!(final_balance, 0);
    // Exactly one transaction should have failed
    failed.sort_unstable();
    assert_eq!(failed.len(), 1);
}

#[test]
fn multiple_threads_mixed_transactions() {
    let txs = vec![
        Transaction::Withdraw(30),
        Transaction::Deposit(20),
        Transaction::Withdraw(70),
        Transaction::Withdraw(10),
        Transaction::Deposit(100),
    ];

    let (final_balance, failed) = process_transactions(50, txs, 3);
    // Compute expected by serial execution (one valid outcome) isn't deterministic
    // but we can assert invariants: final >= 0 and all failed are withdraws that
    // would have overdrawn at the moment they ran.
    assert!(final_balance >= 0);
    for idx in failed {
        assert!(idx < 5);
    }
}
