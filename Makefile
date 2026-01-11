.PHONY: build check clippy fmt all

build:
	cargo build --release

check:
	cargo check --all-targets --all-features

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

fmt:
	cargo fmt --all -- --check && cargo +nightly fmt --all -- --check

all: fmt check clippy build

