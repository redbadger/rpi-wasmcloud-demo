APP = wasm_oled_actor
COLOR ?= always # Valid COLOR options: {always, auto, never}
CARGO = cargo --color $(COLOR)
TARGET = wasm32-wasi
DEBUG = target/$(TARGET)/debug
KEYDIR ?= .keys

.PHONY: all bench build keys keys-account keys-module

all: build

build:
	@$(CARGO) build --target $(TARGET)
	wascap sign $(DEBUG)/$(APP).wasm $(DEBUG)/$(APP)_signed.wasm -i $(KEYDIR)/account.nk -u $(KEYDIR)/module.nk -s -n $(APP)
	wash claims inspect $(DEBUG)/$(APP)_signed.wasm
	
keys: keys-account keys-module

keys-account:
	@mkdir -p $(KEYDIR)
	nk gen account > $(KEYDIR)/account.txt
	awk '/Seed/{ print $$2 }' $(KEYDIR)/account.txt > $(KEYDIR)/account.nk

keys-module:
	@mkdir -p $(KEYDIR)
	nk gen module > $(KEYDIR)/module.txt
	awk '/Seed/{ print $$2 }' $(KEYDIR)/module.txt > $(KEYDIR)/module.nk
