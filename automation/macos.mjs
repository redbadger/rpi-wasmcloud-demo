#!/usr/bin/env zx

import { step } from "./lib.mjs";

const REGISTRY = "registry:5001";
const ACTOR = {
    id: "MC5QO34YH43RO6R3AMM3I4XC7ET2KXEMXLW4CX3XFQR4XWGF6QREPPBH",
    ref: `${REGISTRY}/oled_actor:0.0.1`,
};
const HTTPSERVER = {
    id: "VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M",
    ref: "wasmcloud.azurecr.io/httpserver:0.14.8",
    contract: "wasmcloud:httpserver",
    config: `config_b64=${btoa(JSON.stringify({ address: "0.0.0.0:8080" }))}`,
};
const OLED = {
    id: "VA2JVYCIHAVHFXVDFOQHDYDKONQEBVC7XNMGV6VYHDO2XRR5552GVRUM",
    ref: `${REGISTRY}/oled-provider:0.0.1`,
    contract: "redbadger:oled",
};

if (argv.up) {
    step("Starting containers");
    // await $`tilt up`;
    await $`docker-compose up -d`;
}

if (argv.start) {
    step("starting workloads");

    // pi-01
    await $`wash ctl link put ${ACTOR.id} ${OLED.id} ${OLED.contract}`;
    await $`wash ctl start provider ${OLED.ref} --link-name default --timeout-ms 30000 --constraint node=pi-01`;

    // pi-02
    await $`wash ctl start actor ${ACTOR.ref} --timeout-ms 30000 --constraint node=pi-02`;

    // MacOS
    await $`wash ctl link put ${ACTOR.id} ${HTTPSERVER.id} ${HTTPSERVER.contract} ${HTTPSERVER.config}`;
    await $`wash ctl start provider ${HTTPSERVER.ref} --link-name default --timeout-ms 30000 --constraint node=MacOS`;
}

if (argv.stop) {
    step("stopping workloads");

    const verbosity = $.verbose;
    $.verbose = false;
    const { hosts } = JSON.parse(await $`wash ctl get hosts -o json`);
    const idByNode = {};
    for (const host of hosts) {
        let { inventory } = JSON.parse(
            await $`wash ctl get inventory ${host.id} -o json`
        );
        idByNode[inventory.labels.node] = host.id;
    }
    $.verbose = verbosity;

    await $`wash ctl stop actor ${idByNode["pi-02"]} ${ACTOR.id} --timeout-ms 30000`;
    await $`wash ctl stop provider ${idByNode["pi-01"]} ${OLED.id} default ${OLED.contract} --timeout-ms 30000`;
    await $`wash ctl stop provider ${idByNode["MacOS"]} ${HTTPSERVER.id} default ${HTTPSERVER.contract} --timeout-ms 30000`;
    await $`wash ctl link del ${ACTOR.id} ${OLED.contract}`;
    await $`wash ctl link del ${ACTOR.id} ${HTTPSERVER.contract}`;
}

if (argv.down) {
    step("stopping containers");
    // await $`tilt down`;
    await $`docker-compose down`;
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
                                family === "IPv4" && {
                                    address,
                                    interface: names[i],
                                }) ||
                                []
                        ),
                    []
                )
            ),
        []
    );
    console.log(info);
}
