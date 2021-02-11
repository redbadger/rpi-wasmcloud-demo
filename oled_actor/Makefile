APP = oled_actor
COLOR ?= always # Valid COLOR options: {always, auto, never}
CARGO = cargo --color $(COLOR)
TARGET = wasm32-unknown-unknown
DEBUG = target/$(TARGET)/debug

.PHONY: build clean

build:
	@$(CARGO) build --target $(TARGET)
	wash claims sign \
		--http_server \
		--cap red-badger:oled-ssd1306 \
		--name $(APP) \
		$(DEBUG)/$(APP).wasm
	wash claims inspect $(DEBUG)/$(APP)_s.wasm

# Rust builds accrue disk space over time (specifically the target directory),
# so running `make clean` should be done periodically.
clean:
	cargo clean
	rm -Rf build
