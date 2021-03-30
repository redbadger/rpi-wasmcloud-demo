# Raspberry Pi wasmcloud Demo

This is a demo of a wasmcloud [lattice](https://www.wasmcloud.dev/reference/lattice) running across an Apple MacBook Pro and 2 x Raspberry Pi 4B.

> _A lattice is a seamless, distributed unit of compute that can self-form atop any combination of cloud, edge, and physical infrastructure._

In this example, the lattice is made of three [wasmcloud](https://wasmcloud.dev/) nodes, one on the Mac and one on each Pi. However it would work just as well with one Pi, simply collapse `pi_01` and `pi_02` together as you go.

The Mac node hosts the wasmcloud [HTTP Server provider](https://github.com/wasmCloud/capability-providers) that forwards incoming requests to our sandboxed [WASM](https://webassembly.org/) actor, which can run on any node, but in this demo runs on `pi_02`.

The Mac node also hosts the wasmcloud [Logging provider](https://github.com/wasmCloud/capability-providers), which the actor uses to log to `stdout`.

The WASM actor contains our "business" logic. It is signed and only given permissions to talk with the HTTP Server provider, the Logging provider and the OLED provider. The OLED provider is dynamically linked at runtime into the node running on `pi_01`, where it natively controls an OLED display.

![wasmcloud lattice across Mac and Pi](./docs/wasmcloud-lattice.svg)

## Setup

1. Raspberry Pi 4B, 8GB

   1. Rust stable
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

   1. [Remote SSH](https://code.visualstudio.com/docs/remote/ssh) - useful for writing code directly on a Pi.
   2. [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) - essential :-)

5. wasmcloud and [`wash`](https://github.com/wascc/wash) installed on the Mac:

   ```sh
   cargo install --git https://github.com/wasmcloud/wasmcloud --tag=v0.16.0 wasmcloud
   cargo install wash-cli

   # or
   brew tap wasmcloud/wasmcloud
   brew install wasmcloud wash
   ```

## Build

Build the actor and the provider and push them to an OCI registry.

### `oled_ssd1306`

```sh
cd oled_ssd1306

make

export REGISTRY=redbadger.azurecr.io # set your OCI registry
export OCI_REGISTRY_USER=username # set your OCI registry username
export OCI_REGISTRY_PASSWORD=password # set your OCI registry password
make push
```

### `oled_actor`

```sh
cd oled_actor

make

export REGISTRY=redbadger.azurecr.io # set your OCI registry
export OCI_REGISTRY_USER=username # set your OCI registry username
export OCI_REGISTRY_PASSWORD=password # set your OCI registry password
make push
```

## Run

1. Find the IP address of your Mac:

   ```sh
   ifconfig | sed -En 's/127.0.0.1//;s/.*inet (addr:)?(([0-9]*\.){3}[0-9]*).*/\2/p'
   ```

2. Install wasmcloud on Raspberry Pi 64bit debian:

   ```bash
   # install rust toolchain
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   # dev tools
   sudo apt install libssl-dev libclang-dev clang-9

   # wasmcloud
   cargo install --git https://github.com/wasmcloud/wasmcloud --tag=v0.16.0 wasmcloud
   ```

3. On `pi_01` (the Pi with the OLED display):

   The environment variable `KVCACHE_NATS_URL` is also used by the default KV cache provider, to share the cache between nodes.

   ```sh
   export OCI_REGISTRY_USER=username # set your OCI registry username
   export OCI_REGISTRY_PASSWORD=password # set your OCI registry password
   export KVCACHE_NATS_URL=192.168.121.141 # set your NATS server IP address from step 1
   wasmcloud --control-host $KVCACHE_NATS_URL --rpc-host $KVCACHE_NATS_URL --allow-live-updates --label name=pi-01
   ```

4. On `pi_02` (the other Pi):

   ```sh
   export OCI_REGISTRY_USER=username # set your OCI registry username
   export OCI_REGISTRY_PASSWORD=password # set your OCI registry password
   export KVCACHE_NATS_URL=192.168.121.141 # set your NATS server IP address from step 1
   wasmcloud --control-host $KVCACHE_NATS_URL --rpc-host $KVCACHE_NATS_URL --allow-live-updates --label name=pi-02
   ```

5. On `MacOS`:

   `RUST_LOG=info` is needed for the Logging provider (which our actor uses to log to `stdout`).

   ```sh
   KVCACHE_NATS_URL=0.0.0.0 RUST_LOG=info wasmcloud
   ```

   There is a [start script](./scripts/start.sh), which starts providers and actors, and links them together.

   (You can actually run more than one instance of the actor — each will be scheduled on a free node — and each actor instance logs a unique uuid so we can see in the host logs how requests are load balanced between each of the actors.)

   ```sh
   ./scripts/start.sh

   # to test
   curl -d 'Hello from wasmcloud!' http://127.0.0.1:8081
   curl -X DELETE http://127.0.0.1:8081
   ```

![Photo of setup](docs/wasmcloud.jpg)
