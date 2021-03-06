build:
	cargo build

install:
	true 

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

start:
	cargo run