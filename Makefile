.PHONY: all test clean

install-dev:
	rustup component add clippy
	cargo install cargo-watch

clean:
	cargo clean

lint:
	cargo clippy

fix:
	cargo clippy --fix
	cargo fmt

build:
	cargo build

# use tee to output to file and stdout for debug
dev:
	-rm output.csv
	cargo run -- ./data/transactions.csv | tee output.csv

dev-full:
	-rm output.csv
	cargo run -- ./data/full/transactions.csv | tee output.csv

watch:
	cargo watch -x run  -- ./data/transactions.csv | tee output.csv

test:
	cargo test

bench:
	CRITERION_DEBUG=1 cargo bench

