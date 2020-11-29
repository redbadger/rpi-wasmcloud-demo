# Raspberry Pi WaSCC Demo

This is a demo of a WaSCC [lattice](https://wascc.dev/docs/lattice/overview/) across Apple Mac and Raspberry Pi.

> _A lattice is a seamless, distributed unit of compute that can self-form atop any combination of cloud, edge, and physical infrastructure._

The lattice is made of two [WaSCC](https://wascc.dev/) nodes, one on the Mac and the other on the Pi.

The Mac node hosts an HTTP server provider that forwards incoming requests to a sandboxed [WASM](https://webassembly.org/) actor, which can run on any node.

The WASM actor contains our "business" logic. It is signed and only given permissions to talk with the HTTP server provider and the OLED provider. The latter is dynamically linked into the node running on the Raspberry Pi, where it natively controls an OLED display.

![WaSCC lattice across Mac and Pi](./docs/wascc-lattice.svg)

## The setup

1. Raspberry Pi 4B, 8GB

   1. Rust `1.50.0-nightly`
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

1. Clone this repo on both the Pi and your Mac.

2. Find the IP address of your Mac:

   ```sh
   ipconfig getifaddr en0
   ```

3. Connect VSCode to the Pi over SSH (_cmd-shift-P_ then `Remote-SSH: Connect to Host`) or connect over SSH with a terminal.

4. On the Pi:

   ```sh
   (cd pi_oled_provider && make)
   (cd wasm_oled_actor && make)
   (cd pi_host && make NATS_IP=192.168.121.180)  # set NATS_IP to the IP of your Mac (see step 2)
   ```

5. On the Mac:

   ```sh
   (cd wasm_oled_actor && make)
   (cd mac_host && make)
   ```

6. To test it out:

   ```sh
   curl -d 'Hello from WaSCC!' http://localhost:8081
   curl -X DELETE http://localhost:8081
   ```

7. To run the actor on the Pi instead of the Mac:

   ```sh
   # on the Pi
   (cd pi_host && make NATS_IP=192.168.121.180 ARGS=--actor)  # set NATS_IP to the IP of your Mac (see step 2)
   # on the Mac
   (cd mac_host && make ARGS=)
   ```
