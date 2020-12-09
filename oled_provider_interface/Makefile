.PHONY: build codegen clean

build:
	cargo build

codegen:
	wapc generate codegen.yaml

# Rust builds accrue disk space over time (specifically the target directory),
# so running `make clean` should be done periodically.
clean:
	cargo clean
	rm -Rf build
