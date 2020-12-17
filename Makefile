.PHONY: format build test clean-test clean

default: build

format:
	rustfmt src/*

build:
	cargo build --release

test: test-clean
	cargo test

test-clean:
	rm -rf local out

clean: test-clean
	rm -rf target
