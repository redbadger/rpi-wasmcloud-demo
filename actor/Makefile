# Makefile for oled_actor

PROJECT  = oled_actor
VERSION  = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] .version' | head -1)
REVISION = 0
# list of all contract claims for actor signing (space-separated)
CLAIMS = \
	red-badger:oled-ssd1306 \
	wasmcloud:builtins:logging \
	wasmcloud:builtins:numbergen \
	wasmcloud:httpserver
# registry url for our actor
REG_URL  = localhost:5000/v2/$(PROJECT):$(VERSION)
# command to upload to registry (without last wasm parameter)
PUSH_REG_CMD = wash reg push --insecure $(REG_URL)

# friendly name for the actor
ACTOR_NAME = "oled_actor"
# optional call alias for actor
# ACTOR_ALIAS=nickname

include ./actor.mk
