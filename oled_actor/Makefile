VENDOR ?= red-badger
CAPABILITY_ID := $(VENDOR):oled-ssd1306

COLOR ?= always # Valid COLOR options: {always, auto, never}
CARGO = cargo --color $(COLOR)
NAME := $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].name')
VERSION := $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')
TARGET = wasm32-unknown-unknown
OUT_DIR = target/$(TARGET)/release

.PHONY: build push clean

build:
	$(CARGO) build --target $(TARGET) --release
	wash claims sign \
		--extras \
		--http_server \
		--logging \
		--cap $(CAPABILITY_ID) \
		--name $(NAME) \
		$(OUT_DIR)/$(NAME).wasm
	wash claims inspect $(OUT_DIR)/$(NAME)_s.wasm

push: guard-OCI_REGISTRY_USER guard-OCI_REGISTRY_PASSWORD guard-REGISTRY
	wash reg push \
		-u $(OCI_REGISTRY_USER) \
		-p $(OCI_REGISTRY_PASSWORD) \
		$(REGISTRY)/$(NAME):$(VERSION) \
		$(OUT_DIR)/$(NAME)_s.wasm

# Rust builds accrue disk space over time (specifically the target directory),
# so running `make clean` should be done periodically.
clean:
	cargo clean
	rm -Rf build

guard-%: GUARD
	@ if [ -z '${${*}}' ]; then echo 'Environment variable $* not set.' && exit 1; fi

.PHONY: GUARD
GUARD:
