#!/usr/bin/env zx

import { step } from "./lib.mjs";

const REGISTRY = "registry:5001";
const ACTOR = {
  id: "MC5QO34YH43RO6R3AMM3I4XC7ET2KXEMXLW4CX3XFQR4XWGF6QREPPBH",
  ref: `${REGISTRY}/oled_actor:0.1.1`,
};
const HTTPSERVER = {
  id: "VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M",
  ref: "wasmcloud.azurecr.io/httpserver:0.14.7",
  contract: "wasmcloud:httpserver",
  config: `config_b64=${btoa(JSON.stringify({ address: "0.0.0.0:8080" }))}`,
};
const OLED = {
  id: "VCLB2N33XBBAVPVHIARI7JJJSO4SC2GVAWQ43EKHTPHKKFUCI5OLZQ2Q",
  ref: `${REGISTRY}/oled-ssd1306-provider:0.1.0`,
  contract: "red-badger:oled-ssd1306",
};

if (argv.up) {
  step("Starting containers");
  await $`tilt up`;
}

if (argv.start) {
  step("starting workloads");
  await $`wash ctl start actor ${ACTOR.ref} --timeout-ms 30000 --constraint node=pi-01`;

  await $`wash ctl link put ${ACTOR.id} ${HTTPSERVER.id} ${HTTPSERVER.contract} ${HTTPSERVER.config}`;
  await $`wash ctl start provider ${HTTPSERVER.ref} --link-name default --timeout-ms 30000 --constraint node=MacOS`;

  await $`wash ctl link put ${ACTOR.id} ${OLED.id} ${OLED.contract}`;
  await $`wash ctl start provider ${OLED.ref} --link-name default --timeout-ms 30000 --constraint node=pi-02`;
}

if (argv.stop) {
  step("stop workloads");
  const host = await getHost();
  await $`wash ctl stop actor ${host} ${ACTOR.id} --timeout-ms 30000`;
  await $`wash ctl stop provider ${host} ${OLED.id} default ${OLED.contract} --timeout-ms 30000`;
  await $`wash ctl stop provider ${host} ${HTTPSERVER.id} default ${HTTPSERVER.contract} --timeout-ms 30000`;
  await $`wash ctl link del ${ACTOR.id} ${OLED.contract}`;
  await $`wash ctl link del ${ACTOR.id} ${HTTPSERVER.contract}`;
}

if (argv.down) {
  step("stopping containers");
  await $`tilt down`;
}

if (argv.ip) {
  const interfaces = os.networkInterfaces();
  const names = Object.keys(interfaces);
  const info = Object.values(interfaces).reduce(
    (r, list, i) =>
      r.concat(
        list.reduce(
          (rr, { family, internal, address }) =>
            rr.concat(
              (!internal &&
                family === "IPv4" && { address, interface: names[i] }) ||
                []
            ),
          []
        )
      ),
    []
  );
  console.log(info);
}
