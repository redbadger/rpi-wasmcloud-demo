CONST_FILE := oled_ssd1306_provider/src/lib.rs
define readvar
$(or $(shell sed -nE 's/^.*$(1).*"(.*)";$$/\1/p' $(CONST_FILE)),$(error "Cannot find variable $(1)"))
endef

CAPABILITY_ID := $(call readvar,CAPABILITY_ID)
NAME          := $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[1].name')
VERSION       := $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[1].version')
LIBNAME       := $(shell echo $(NAME) | tr '-' '_')
VENDOR        ?= red-badger
REVISION	  ?= 0
DESTINATION   ?= $(NAME).par.gz

TARGETS := armv7-unknown-linux-gnueabihf

.PHONY: $(TARGETS)
$(TARGETS):
	cross build -p oled_ssd1306_provider --target $@ --release

.PHONY: par
par: $(TARGETS)
	cd build && wash par create \
		--arch aarch64-linux \
		--binary ../target/armv7-unknown-linux-gnueabihf/release/lib$(LIBNAME).so \
		--capid $(CAPABILITY_ID) \
		--name $(NAME) \
		--vendor $(VENDOR) \
		--version $(VERSION) \
		--revision $(REVISION) \
		--destination $(DESTINATION) \
		--compress

.PHONY: codegen
codegen:
	cd oled_ssd1306_interface && wapc generate codegen.yaml

.PHONY: interface
interface:
	cargo build -p oled_ssd1306_interface --target wasm32-unknown-unknown --release
	mkdir -p build && cp target/wasm32-unknown-unknown/release/*.wasm build/

.PHONY: clean
clean:
	cargo clean
	rm -Rf oled_ssd1306_interface/build

.PHONY: test
test: build
	cargo -p oled_ssd1306_interface test
