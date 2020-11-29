ARGS ?=
NATS_IP ?= 192.168.121.180

.PHONY: run

run:
	LATTICE_HOST=$(NATS_IP) RUST_LOG=crane=warn,regalloc=warn,info cargo run -- $(ARGS)
