# Preventing Data Races in Rust

Rust provides inherent protection against data races and thread safety issues at compile time.

## Instructions

1. Examine the code in `main.rs` that tries to spawn threads and mutate a shared variable.

2. Try compiling with `cargo build`. Note that it fails with an error about mutable borrows.

3. Rust disallows this unsafe code by design. The borrow checker prevents aliased mutation.

4. Contrast this with the thread safe mutex version that compiles and runs correctly.

## Challenges

1. Experiment by adding different thread safety bugs like concurrent writes. See errors.

2. Learn about Rust concurrency patterns like Mutex, Arc, Channels. Implement solutions.

3. Use Rust analyzers in IDE to detect issues around threads and shared state.

4. Set up Clippy linter and deny warnings for detectable concurrency errors.

5. Refactor code to minimize shared mutable state across threads.