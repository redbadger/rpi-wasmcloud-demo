# Raspberry Pi wasmcloud Demo

This is a demo of a wasmcloud [lattice](https://www.wasmcloud.dev/reference/lattice) running across an Apple MacBook Pro and 2 x Raspberry Pi 4B.

> _A lattice is a seamless, distributed unit of compute that can self-form atop any combination of cloud, edge, and physical infrastructure._

In this example, the lattice is made of three [wasmcloud](https://wasmcloud.dev/) nodes, one on the Mac and one on each Pi. However it would work just as well with one Pi, simply collapse `pi_01` and `pi_02` together as you go.

The Mac node hosts the wasmcloud [HTTP Server provider](https://github.com/wasmCloud/capability-providers) that forwards incoming requests to our sandboxed [WASM](https://webassembly.org/) actor, which can run on any node, but in this demo runs on `pi_02`.

Wasmcloud has a built-in [Logging provider](https://github.com/wasmCloud/capability-providers), which the actor uses to log to `stdout`.

The WASM actor contains our "business" logic. It is signed and only given permissions to talk with the HTTP Server provider, the Logging provider, the NumberGen provider (for generating a uuid to identify the actor instance) and the OLED provider. The OLED provider is hosted by the wasmCloud host running on `pi_01`, where it natively controls an OLED display.

![wasmcloud lattice across Mac and Pi](./docs/wasmcloud-lattice.svg)

## Setup

1. make sure I2C is enabled on the Pi with the Oled display attached.

   ```bash
   sudo raspi-config
   ```

2. install Rust on each Pi

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. install Elixir on each Pi

   ```bash
   echo "deb https://packages.erlang-solutions.com/debian buster contrib" \
   | sudo tee /etc/apt/sources.list.d/erlang-solutions.list

   wget https://packages.erlang-solutions.com/debian/erlang_solutions.asc \
   && sudo apt-key add erlang_solutions.asc \
   && rm erlang_solutions.asc

   sudo apt update
   sudo apt install erlang-parsetools erlang-dev elixir
   ```

4. clone the `wasmcloud-otp` repo, then build `wasmcloud_host` on each Pi

   ```bash
   git clone git@github.com:wasmCloud/wasmcloud-otp.git
   cd wasmcloud-otp/wasmcloud_host
   make build
   ```

5. install the OLED display with SSD1306 display driver, on one Pi

   1. [MakerHawk OLED Display Module, SSD1306, 128x64](https://smile.amazon.co.uk/gp/product/B0777HHQDT)
   2. Header pins need soldering onto the OLED board
   3. Jumper leads to these pins on the Pi:
      1. `VCC` - pin 1
      2. `GND` - pin 6
      3. `SCL` - pin 5
      4. `SDA` - pin 3

6. find the IP address of your Mac

   ```sh
   ifconfig | sed -En 's/127.0.0.1//;s/.*inet (addr:)?(([0-9]*\.){3}[0-9]*).*/\2/p'
   ```

   and add the Mac's IP address and hostname to `/etc/hosts` on each Pi, so that you can use the OCI registry hosted on the mac.

7. run NATS server, wasmcloud, redis, and a local OCI registry, on the Mac

   ```sh
   docker-compose up -d

   # if using `lima` and `containerd`...
   # (note you may need to install `vde_vmnet` as per
   # https://github.com/lima-vm/lima/blob/master/docs/network.md
   # to allow access to NATS from other machines)
   lima nerdctl compose up -d
   ```

   Note the cluster seed and signer keys and add them to `~/wasmcloud-otp/wasmcloud_host/.env` files on each Pi. The `.env` files should look like the following (replace the keys and the mac's host name).

   ```bash
   WASMCLOUD_CLUSTER_SEED=SCADVNVKDODD25EHKQHT4UAAZMKQUXTEO5PFLMPYFZWUCSIC6NPWLLJRJE
   WASMCLOUD_CLUSTER_ISSUERS=CC32VPAIXM7FYMJQKPEA4JJAAQQEFGEHYRU3FOJR4EC7AWIANLIZ3PYB
   WASMCLOUD_CTL_HOST=stuarts-macbook-pro.local
   WASMCLOUD_RPC_HOST=stuarts-macbook-pro.local
   WASMCLOUD_PROV_RPC_HOST=stuarts-macbook-pro.local
   WASMCLOUD_OCI_ALLOWED_INSECURE=stuarts-macbook-pro.local:5000
   ```

8. run a wasmCloud host on each Pi:

   ```bash
   cd ~/wasmcloud-otp/wasmcloud_host
   make run
   ```

9. open the washboard in a browser on the mac (http://localhost:4000) for starting providers, actors and defining links.

10. you may want to install these extensions into vscode (on the Mac and the Pi that you use to build the provider)

    1. [Remote SSH](https://code.visualstudio.com/docs/remote/ssh) - useful for writing code directly on a Pi.
    2. [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) - essential :-)

11. install `wash` on the Mac:

    ```sh
    brew tap wasmcloud/wasmcloud
    brew install wash
    ```

12. install `wash` on the Pi that is used to build the provider:

    ```sh
    cargo install wash-cli
    ```

## Build

Build the provider and the actor, and push them to an OCI registry.

### `provider`

```sh
# on a Raspberry Pi, e.g. via vscode remote
cd provider
make build

# push to an OCI registry, e.g...
wash reg push --insecure stuarts-macbook-pro.local:5000/v2/oled-ssd1306-provider:0.1.0 build/oled-ssd1306-provider.par.gz
```

### `actor`

```sh
# on the MacBook
cd actor
make

# push to an OCI registry, e.g...
wash reg push --insecure stuarts-macbook-pro.local:5000/v2/oled_actor:0.1.0 build/oled_actor_s.wasm
```

## Run

The script ([./scripts/start.sh](./scripts/start.sh)) still needs updating, so for now, use the washboard to start the oled provider (on `pi_01`), http provider (on mac), actor (`pi_02`, or wherever) and create links between them. Note that when creating the link for the http server provider, use `address=0.0.0.0:8081` (or similar) if you are hosting wasmCloud in docker on the mac.

```sh

# to test
curl -d 'Hello from wasmcloud!' http://127.0.0.1:8081
curl -X DELETE http://127.0.0.1:8081
```

![Photo of setup](docs/wasmcloud.jpg)
