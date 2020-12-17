.PHONY: build test clean-test clean

default: build

build:
	cargo build --release

test:
	cargo test

test-clean:
	rm -rf local out

clean: test-clean
	rm -rf target
