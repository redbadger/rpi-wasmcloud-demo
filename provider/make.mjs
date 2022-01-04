#!/usr/bin/env zx

import {
  getArch,
  getProject,
  ifChanged,
  setColors,
  step,
} from "../automation/lib.mjs";

const config = {
  capability: "red-badger:oled-ssd1306",
  vendor: "RedBadger",
  registry: "registry:5001",
};

setColors();

const { name: project, version } = await getProject();
const revision = 0;
const build = argv.debug ? "debug" : "release";

if (argv.clean) {
  step("Cleaning ...");
  await $`cargo clean`;
  await $`rm -rf build`;
}

const destination = `build/${project}.par.gz`;

if (argv.build) {
  await fs.ensureDir("build");

  step("Building provider...");
  await ifChanged([".", "../interface"], "build", async () => {
    await $`cargo build ${build === "release" ? "--release" : ""}`;

    const source = `target/${build}/${project}`;
    await $`wash par create ${[
      "--arch",
      getArch(),
      "--binary",
      source,
      "--capid",
      config.capability,
      "--name",
      project,
      "--vendor",
      config.vendor,
      "--version",
      version,
      "--revision",
      revision,
      "--destination",
      destination,
      "--compress",
    ]}`;
  });

  await $`wash par inspect ${destination}`;
}

if (argv.push) {
  step("Pushing...");
  await $`wash reg push --insecure ${config.registry}/${project}:${version} ${destination}`;
}
