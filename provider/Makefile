CAPABILITY_ID = "red-badger:oled-ssd1306"
NAME = "oled-ssd1306-provider"
VENDOR = "RedBadger"
PROJECT = oled-ssd1306-provider
VERSION = 0.1.0
REVISION = 0

.PHONY: build
build:
	cargo build --release
	wash par create \
		--arch aarch64-linux \
		--binary target/release/$(PROJECT) \
		--capid $(CAPABILITY_ID) \
		--name $(NAME) \
		--vendor $(VENDOR) \
		--version $(VERSION) \
		--revision $(REVISION) \
		--destination build/$(PROJECT).par.gz \
		--compress
	wash par inspect build/$(PROJECT).par.gz
