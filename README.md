# Raspberry Pi wasmcloud Demo

This is a demo of a wasmCloud [lattice](https://www.wasmcloud.dev/reference/lattice) running across an Apple MacBook Pro and 2 x Raspberry Pi 4B.

> _The lattice is a self-forming, self-healing mesh network that provides a unified, flattened topology across any number of disparate environments, clouds, browsers, or even hardware._

In this example, the lattice is made of three [wasmcloud](https://wasmcloud.dev/) nodes, one on the Mac and one on each Pi. However it would work just as well with one Pi, simply collapse `pi_01` and `pi_02` together as you go.

The Mac node hosts the wasmcloud [HTTP Server Capability](https://github.com/wasmCloud/capability-providers) that forwards incoming requests to our sandboxed [Wasm](https://webassembly.org/) actor, which can run on any node, but in this demo runs on `pi_02`.

Wasmcloud has a built-in [Logging Capability](https://github.com/wasmCloud/capability-providers), which the actor uses to log to `stdout`.

The Wasm actor contains our "business" logic. It is signed and only given permissions to talk with the HTTP Server capability, the Logging capability, the NumberGen capability (for generating a uuid to identify the actor instance) and our OLED capability. This OLED capability is hosted by the wasmCloud host running on `pi_01`, where it natively controls an OLED display.

![wasmcloud lattice across Mac and Pi](./docs/wasmcloud-lattice.svg)

## Setup

1. make sure I2C is enabled on `pi-01` (the Pi with the Oled display attached).

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

4. clone the `wasmcloud-otp` repo, then build `host_core` on each Pi

    ```bash
    git clone git@github.com:wasmCloud/wasmcloud-otp.git
    cd wasmcloud-otp
    git checkout v0.52.2
    cd host_core
    make build
    ```

5. install the OLED display with SSD1306 display driver, on `pi-01`

    1. [MakerHawk OLED Display Module, SSD1306, 128x64](https://smile.amazon.co.uk/gp/product/B0777HHQDT)
    2. Header pins need soldering onto the OLED board
    3. Jumper leads to these pins on the Pi:
        1. `VCC` - pin 1
        2. `GND` - pin 6
        3. `SCL` - pin 5
        4. `SDA` - pin 3

6. find the IP address of your Mac (this may list several, in which case choose one on the interface to the subnet containing your Raspberry Pi devices)

    ```sh
    ./automation/macos.mjs --ip
    ```

    and add the Mac's IP address and hostname to `/etc/hosts` on each Pi, so that you can use the OCI registry hosted on the mac.

7. run NATS server, wasmcloud, redis, and a local OCI registry, on the Mac

    ```sh
    ./automation/macos.mjs --up
    ```

8. run a wasmCloud host on each Pi:

    ```bash
    git clone git@github.com:redbadger/rpi-wasmcloud-demo.git
    cd rpi-wasmcloud-demo

    ./automation/rpi.mjs --up
    ```

9. open the washboard in a browser on the mac (<http://localhost:4000>) for starting providers, actors and defining links.

10. you may want to install these extensions into vscode (on the Mac and the Pi that you use to build the provider)

    1. [Remote SSH](https://code.visualstudio.com/docs/remote/ssh) - useful for writing code directly on a Pi.
    2. [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) - essential :-)

11. install `wash` on the Mac:

    ```sh
    # using homebrew ...
    brew tap wasmcloud/wasmcloud
    brew install wash

    # ... or with cargo
    cargo install wash-cli
    ```

12. install `wash` on the Pi that is used to build the provider:

    ```sh
    cargo install wash-cli
    ```

## Build

Build the provider and the actor, and push them to an OCI registry.

### `provider`

```sh
# on a Raspberry Pi, e.g. via ssh, or vscode remote

# install node
curl -fsSL https://fnm.vercel.app/install | bash
source /home/pi/.bashrc

fnm install 16

# install zx
npm i --global zx

# install dirsh
cargo install dirsh

# build provider
./provider/make.mjs --build --push

# push to registry on mac (ensure there is an entry in /etc/hosts for `registry`, pointing at Mac)
make push
```

### `actor`

```sh
# on the MacBook
./actor/make.mjs --build --push
```

## Run

```sh
./automation/macos.mjs --start
```

## Test

```sh
# to test
curl -d 'Hello from wasmcloud!' http://127.0.0.1:8080
curl -X DELETE http://127.0.0.1:8080
```

![Photo of setup](docs/wasmcloud.jpg)
