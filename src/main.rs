mod bank;
mod simple;

fn main() {
    simple::run_simple_safe_threads();
    bank::run_bank_example();
}
