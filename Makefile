format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

# Clean Rust build artifacts
clean:
	cargo clean

test:
	cargo test --quiet

run:
	cargo run

all: format lint test run

