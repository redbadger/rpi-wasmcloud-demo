APP = oled_actor
COLOR ?= always # Valid COLOR options: {always, auto, never}
CARGO = cargo --color $(COLOR)
TARGET = wasm32-unknown-unknown
OUT_DIR = target/$(TARGET)/release

.PHONY: build clean

build:
	$(CARGO) build --target $(TARGET) --release
	wash claims sign \
		--http_server \
		--logging \
		--cap red-badger:oled-ssd1306 \
		--name $(APP) \
		$(OUT_DIR)/$(APP).wasm
	wash claims inspect $(OUT_DIR)/$(APP)_s.wasm

# Rust builds accrue disk space over time (specifically the target directory),
# so running `make clean` should be done periodically.
clean:
	cargo clean
	rm -Rf build
