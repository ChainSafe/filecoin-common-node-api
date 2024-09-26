build:
	cargo build

release:
	cargo build --release

install:
	cargo install

lint:
	cargo fmt --all --check