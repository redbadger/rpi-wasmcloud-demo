VENDOR        ?= red-badger
CAPABILITY_ID := $(VENDOR):oled-ssd1306
NAME          := $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[1].name')
VERSION       := $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[1].version')
LIBNAME       := $(shell echo $(NAME) | tr '-' '_')
REVISION	  ?= 0
DESTINATION   ?= $(NAME).par.gz

TARGETS := aarch64-unknown-linux-gnu

.PHONY: $(TARGETS)
$(TARGETS):
	cross build -p oled-ssd1306-provider --target $@ --release

.PHONY: par
par: $(TARGETS)
	cd target/aarch64-unknown-linux-gnu/release/ && wash par create \
		--arch aarch64-linux \
		--binary lib$(LIBNAME).so \
		--capid $(CAPABILITY_ID) \
		--name $(NAME) \
		--vendor $(VENDOR) \
		--version $(VERSION) \
		--revision $(REVISION) \
		--destination $(DESTINATION) \
		--compress

.PHONY: codegen
codegen:
	cd oled-ssd1306-interface && wapc generate codegen.yaml

.PHONY: clean
clean:
	cargo clean
