# Oled provider

This provider controls an ssd1306 oled display.

It currently only builds for Raspberry Pi `aarch64` — for now, use vscode remote to compile directly on the rpi (will update to use `cross` at some point, but first we need to fix wasmcloud's base image for aarch64 — there is an OpenSSL error: `fatal error: openssl/opensslconf.h: No such file or directory`).
