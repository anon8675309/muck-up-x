.PHONY: all

all: target/debug/muck-up-x

target/debug/muck-up-x: src/main.rs Cargo.toml
	cargo build
