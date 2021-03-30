VENDOR ?= red-badger
CAPABILITY_ID := $(VENDOR):oled-ssd1306
NAME := $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[1].name')
VERSION := $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[1].version')
LIBNAME := $(shell echo $(NAME) | tr '-' '_')
REVISION ?= 0
DESTINATION ?= $(NAME).par.gz

TARGETS := aarch64-unknown-linux-gnu

default: par

.PHONY: $(TARGETS)
$(TARGETS):
	cross build -p $(NAME) --target $@ --release

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

push: guard-OCI_REGISTRY_USER guard-OCI_REGISTRY_PASSWORD guard-REGISTRY
	wash reg push \
		-u $(OCI_REGISTRY_USER) \
		-p $(OCI_REGISTRY_PASSWORD) \
		$(REGISTRY)/$(LIBNAME):$(VERSION) \
		target/aarch64-unknown-linux-gnu/release/$(NAME).par.gz

.PHONY: clean
clean:
	cargo clean

guard-%: GUARD
	@ if [ -z '${${*}}' ]; then echo 'Environment variable $* not set.' && exit 1; fi

.PHONY: GUARD
GUARD:
