#!/usr/bin/env zx

import { ifChanged, setColors, step } from "../automation/lib.mjs";

setColors();
const build = argv.debug ? "debug" : "release";

if (argv.clean) {
    step("Cleaning...");
    cd("rust");
    await $`cargo clean`;
    await $`rm -rf build`;
}

if (argv.build) {
    step("Building...");
    await fs.ensureDir("rust/build");
    cd("rust");
    await ifChanged(
        ".",
        "build",
        () => $`cargo build ${build === "release" ? "--release" : ""}`
    );
}
