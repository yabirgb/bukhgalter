build:
	cargo build 

test: 
	cargo test

run: 
	cargo run

release:
	cargo build --release	

docs:
	cargo doc --no-deps

check:
	cargo check