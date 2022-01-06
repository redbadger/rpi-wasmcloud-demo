#!/usr/bin/env zx

import { step } from "./lib.mjs";

const ENV = {
  WASMCLOUD_CLUSTER_SEED:
    "SCANP3E75PCKS5AF2UI56HBJ5HVGYVXL52ZJS35S6MVHOYB7LAAXSU6B24",
  WASMCLOUD_CLUSTER_ISSUERS:
    "CDAM4OLLU5ZKQTWXYCGJ2IKMAFIHFCXTBIEOAGUDK26KUVJAH3RCXGUS",
  WASMCLOUD_CTL_HOST: "stuarts-macbook-pro.local",
  WASMCLOUD_RPC_HOST: "stuarts-macbook-pro.local",
  WASMCLOUD_PROV_RPC_HOST: "stuarts-macbook-pro.local",
  WASMCLOUD_OCI_ALLOWED_INSECURE: "registry:5001",
  HOST_NODE: os.hostname(),
};

if (argv.up) {
  step("Starting wasmcloud");
  cd("~/wasmcloud-otp/host_core");
  await $`
  	${[...Object.keys(ENV).map((k) => `${k}=${ENV[k]}`)]}
	make run-interactive`;
}
