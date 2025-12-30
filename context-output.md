# Project Context

**Language**: rust
**Project Path**: .

## Project Structure

- **Total Files**: 4
- **Total Functions**: 4
- **Median Cyclomatic**: 2.00
- **Median Cognitive**: 1.00

## Quality Scorecard

- **Overall Health**: 85.0%
- **Maintainability Index**: 70.0
- **Complexity Score**: 100.0
- **Test Coverage**: 65.0%

## Files

### ./src/bank.rs

**File Complexity**: 2 | **Functions**: 1

- **Function**: `run_bank_example` [complexity: 2] [cognitive: 1] [big-o: O(1)] [provability: 42%] [satd: 0] [churn: low(1)] [tdg: 2.5]

### ./src/lib.rs

**File Complexity**: 6 | **Functions**: 1

- **Enum**: `Transaction` [variants: 2]
- **Function**: `process_transactions` [complexity: 6] [cognitive: 8] [big-o: O(n)] [provability: 42%] [satd: 0] [churn: low(1)] [tdg: 2.5]
- **Function**: `tests::deposits_and_withdrawals_single_thread` [complexity: 3] [cognitive: 2] [big-o: O(n)] [provability: 42%] [satd: 0] [churn: low(1)] [tdg: 2.5]
- **Function**: `tests::prevents_overdraft_across_threads` [complexity: 3] [cognitive: 2] [big-o: O(n)] [provability: 42%] [satd: 0] [churn: low(1)] [tdg: 2.5]
- **Function**: `tests::multiple_threads_mixed_transactions` [complexity: 3] [cognitive: 2] [big-o: O(n)] [provability: 42%] [satd: 0] [churn: low(1)] [tdg: 2.5]

### ./src/main.rs

**File Complexity**: 1 | **Functions**: 1

- **Function**: `main` [complexity: 1] [cognitive: 0] [big-o: O(1)] [provability: 42%] [satd: 0] [churn: low(3)] [tdg: 2.5]

### ./src/simple.rs

**File Complexity**: 2 | **Functions**: 1

- **Function**: `run_simple_safe_threads` [complexity: 2] [cognitive: 1] [big-o: O(1)] [provability: 42%] [satd: 0] [churn: low(1)] [tdg: 2.5]


