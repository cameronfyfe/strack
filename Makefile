.PHONY: test clean

default: all

build:
	cargo build

build-release:
	cargo build --release

all: build

test:
	cargo test

clean-test:
	rm -rf local out

clean: clean-test
	rm -rf target
