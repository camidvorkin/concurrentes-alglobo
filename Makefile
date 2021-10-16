build:
	cargo build

run: 
	cargo run

linter:
	cargo clippy
	cargo fmt

doc:
	cargo doc --no-deps --target-dir docs