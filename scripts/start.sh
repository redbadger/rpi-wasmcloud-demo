#!/bin/bash

set -euo pipefail

ACTOR=MC5QO34YH43RO6R3AMM3I4XC7ET2KXEMXLW4CX3XFQR4XWGF6QREPPBH
PROVIDER_EXTRAS=VDHPKGFKDI34Y4RN4PWWZHRYZ6373HYRSNNEM4UTDLLOGO5B37TSVREP
PROVIDER_HTTP=VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M
PROVIDER_LOGGING=VCCANMDC7KONJK435W6T7JFEEL7S3ZG6GUZMZ3FHTBZ32OZHJQR5MJWZ
PROVIDER_OLED=VCLB2N33XBBAVPVHIARI7JJJSO4SC2GVAWQ43EKHTPHKKFUCI5OLZQ2Q

# Capability providers...
wash ctl start provider redbadger.azurecr.io/oled_ssd1306_provider:0.0.1 --constraint "name=pi-01" || true
wash ctl start provider wasmcloud.azurecr.io/httpserver:0.11.1 --constraint "hostcore.os=macos" || true
wash ctl start provider wasmcloud.azurecr.io/logging:0.9.1 --constraint "hostcore.os=macos" || true

# Actors...
wash ctl start actor redbadger.azurecr.io/oled_actor:0.0.1 --constraint "name=pi-02" || true

# Links...
wash ctl link $ACTOR $PROVIDER_EXTRAS wasmcloud:extras
wash ctl link $ACTOR $PROVIDER_HTTP wasmcloud:httpserver PORT=8081
wash ctl link $ACTOR $PROVIDER_LOGGING wasmcloud:logging
wash ctl link $ACTOR $PROVIDER_OLED red-badger:oled-ssd1306
