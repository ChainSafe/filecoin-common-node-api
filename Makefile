build:
	cargo build

release:
	cargo build --release

install:
	cargo install

lint:
	cargo fmt --all --check

clippy:
	cargo clippy --all-targets --quiet --no-deps

test:
	cargo test

check:
	cargo check

md-fmt:
	corepack enable && yarn && yarn md-fmt

md-check:
	corepack enable && yarn && yarn md-check
