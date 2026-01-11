.PHONY: build check clippy fmt all

build:
	cargo build

check:
	cargo check

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

fmt:
	cargo fmt; cargo +nightly fmt --all

all: fmt check clippy build 
