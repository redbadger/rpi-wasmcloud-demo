# Raspberry Pi WaSCC Demo

This is a demo of a WaSCC [lattice](https://wascc.dev/docs/lattice/overview/) across Apple Mac and Raspberry Pi.

> _A lattice is a seamless, distributed unit of compute that can self-form atop any combination of cloud, edge, and physical infrastructure._

The lattice is made of two [wasmCloud](https://wasmcloud.dev/) nodes, one on the Mac and the other on the Pi.

The Mac node hosts an HTTP server provider that forwards incoming requests to a sandboxed [WASM](https://webassembly.org/) actor, which can run on any node.

The WASM actor contains our "business" logic. It is signed and only given permissions to talk with the HTTP server provider and the OLED provider. The latter is dynamically linked into the node running on the Raspberry Pi, where it natively controls an OLED display.

![wasmcloud lattice across Mac and Pi](./docs/wasmcloud-lattice.svg)

## The setup

1. Raspberry Pi 4B, 8GB

   1. Rust `1.50`
   2. Rust Analyzer – `aarch64` builds are currently only available on nightly (`rustup component add rust-analyzer-preview`)
   3. I2C enabled in `sudo raspi-config`

2. OLED display with SSD1306 display driver

   1. [MakerHawk OLED Display Module, SSD1306, 128x64](https://smile.amazon.co.uk/gp/product/B0777HHQDT)
   2. Header pins need soldering onto the OLED board
   3. Jumper leads to these pins on the Pi:
      1. `VCC` - pin 1
      2. `GND` - pin 6
      3. `SCL` - pin 5
      4. `SDA` - pin 3

3. NATS server on the Mac:

   ```sh
   brew install nats-server
   brew services start nats-server
   ```

4. VSCode with these extensions

   1. [Remote SSH](https://code.visualstudio.com/docs/remote/ssh)
   2. [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)

5. [`wash`](https://github.com/wascc/wash) cli installed on the Mac:

   ```sh
   cargo install wash-cli
   ```

## Run it

1. Find the IP address of your Mac:

   ```sh
   ipconfig getifaddr en0
   ```

2. Connect VSCode to the Pi over SSH (_cmd-shift-P_ then `Remote-SSH: Connect to Host`) or connect over SSH with a terminal.

3. Install wasmCloud on Raspberry Pi 64bit

   ```bash
   # install rust toolchain
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   # dev tools
   sudo apt-get install libssl-dev clang-9

   # wasmCloud
   cargo install --git https://github.com/wasmcloud/wasmcloud --branch=main
   ```

4. On the Pi:

   ```sh
   wasmcloud --control-host 192.168.2.1 --rpc-host 192.168.2.1 # set IP addresses to the IP of your Mac (see step 2)
   ```

5. On the Mac:

   ```sh
   wasmcloud

   # use wash cli to load actors and providers TODO

   # to test
   curl -d 'Hello from wasmCloud!' http://localhost:8081
   curl -X DELETE http://localhost:8081
   ```
