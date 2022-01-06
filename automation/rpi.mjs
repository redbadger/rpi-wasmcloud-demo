#!/usr/bin/env zx

import { step } from "./lib.mjs";

process.env.WASMCLOUD_CLUSTER_SEED =
  "SCANP3E75PCKS5AF2UI56HBJ5HVGYVXL52ZJS35S6MVHOYB7LAAXSU6B24";
process.env.WASMCLOUD_CLUSTER_ISSUERS =
  "CDAM4OLLU5ZKQTWXYCGJ2IKMAFIHFCXTBIEOAGUDK26KUVJAH3RCXGUS";
process.env.WASMCLOUD_CTL_HOST = "stuarts-macbook-pro.local";
process.env.WASMCLOUD_RPC_HOST = "stuarts-macbook-pro.local";
process.env.WASMCLOUD_PROV_RPC_HOST = "stuarts-macbook-pro.local";
process.env.WASMCLOUD_OCI_ALLOWED_INSECURE = "registry:5001";
process.env.HOST_NODE = os.hostname();

if (argv.up) {
  step("Starting wasmcloud");
  cd(`${os.homedir()}/wasmcloud-otp/host_core`); // TODO: make this better :-)
  await $`make run-interactive`;
}
