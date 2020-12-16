.PHONY: build build-release build-tests test clean-test clean

default: all

build:
	cargo build

build-release:
	cargo build --release

build-tests:
	cargo build --tests

all: build

test:
	cargo test

clean-test:
	rm -rf local out

clean: clean-test
	rm -rf target
