#!/usr/bin/env zx

import { step } from "./lib.mjs";

const REGISTRY = "registry:5001";
const WASMCLOUD_CLUSTER_SEED =
  "SCANP3E75PCKS5AF2UI56HBJ5HVGYVXL52ZJS35S6MVHOYB7LAAXSU6B24";
const WASMCLOUD_CLUSTER_ISSUERS =
  "CDAM4OLLU5ZKQTWXYCGJ2IKMAFIHFCXTBIEOAGUDK26KUVJAH3RCXGUS";
const ACTOR = {
  id: "MC5QO34YH43RO6R3AMM3I4XC7ET2KXEMXLW4CX3XFQR4XWGF6QREPPBH",
  ref: `${REGISTRY}/oled_actor:0.0.1`,
};
const HTTPSERVER = {
  id: "VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M",
  ref: "wasmcloud.azurecr.io/httpserver:0.14.7",
  contract: "wasmcloud:httpserver",
  config: `config_b64=${btoa(JSON.stringify({ address: "0.0.0.0:8080" }))}`,
};
const OLED = {
  id: "VCLB2N33XBBAVPVHIARI7JJJSO4SC2GVAWQ43EKHTPHKKFUCI5OLZQ2Q",
  ref: `${REGISTRY}/oled_ssd1306_provider:0.0.1`,
  contract: "red-badger:oled-ssd1306",
};

if (argv.up) {
  step("Starting containers");
  await $`
  	WASMCLOUD_CLUSTER_SEED=${WASMCLOUD_CLUSTER_SEED} \
	WASMCLOUD_CLUSTER_ISSUERS=${WASMCLOUD_CLUSTER_ISSUERS} \
	docker compose up -d`;
}

if (argv.start) {
  step("starting workloads");
  //   await $`(cd ../actor && make push)`;
  //   await $`(cd ../provider && make push)`;
  //   await $`wash ctl start actor ${ACTOR.ref} --timeout 30`;

  await $`wash ctl link put ${ACTOR.id} ${HTTPSERVER.id} ${HTTPSERVER.contract} ${HTTPSERVER.config}`;
  //   await $`wash ctl start provider ${HTTPSERVER.ref} --link-name default --timeout 30`;

  await $`wash ctl link put ${ACTOR.id} ${OLED.id} ${OLED.contract}`;
  //   await $`wash ctl start provider ${OLED.ref} --link-name default --timeout 30`;
}

if (argv.restart_actor) {
  step("restarting actor");
  const host = await getHost();
  await $`wash ctl stop actor ${host} ${ACTOR.id} --timeout 30`;
  await $`wash drain all`;
  await $`(cd ../actor && make push)`;
  await $`wash ctl start actor ${ACTOR.ref} --timeout 30`;
}

if (argv.restart_provider) {
  step("restarting provider");
  const host = await getHost();
  await $`wash ctl stop provider ${host} ${OLED.id} default ${OLED.contract} --timeout 30`;
  await $`wash drain all`;
  await $`(cd ../provider && make push)`;
  await $`wash ctl start provider ${OLED.ref} --link-name default --timeout 30`;
}

if (argv.stop) {
  step("stop workloads");
  const host = await getHost();
  await $`wash ctl stop actor ${host} ${ACTOR.id} --timeout 30`;
  await $`wash ctl stop provider ${host} ${OLED.id} default ${OLED.contract} --timeout 30`;
  await $`wash ctl stop provider ${host} ${HTTPSERVER.id} default ${HTTPSERVER.contract} --timeout 30`;
  await $`wash ctl link del ${ACTOR.id} ${OLED.contract}`;
  await $`wash ctl link del ${ACTOR.id} ${HTTPSERVER.contract}`;
}

if (argv.down) {
  step("stopping containers");
  await $`docker compose down`;
}

if (argv.ip) {
  const info = Object.values(os.networkInterfaces()).reduce(
    (r, list) =>
      r.concat(
        list.reduce(
          (rr, i) =>
            rr.concat((i.family === "IPv4" && !i.internal && i.address) || []),
          []
        )
      ),
    []
  );
  console.log(info);
}
