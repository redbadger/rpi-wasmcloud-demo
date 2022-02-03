#!/usr/bin/env zx

export async function retry(
    { count, delay = 5000 },
    f,
    evaluator = async () => false
) {
    for (let i = 0; i < count; i++) {
        try {
            return await f();
        } catch (e) {
            let done = await evaluator(e);
            if (done) return e;
            await new Promise((Y) => setTimeout(Y, delay));
        }
    }
}

export async function ifChanged(inputDirs, outputDir, fn) {
    const verbosity = $.verbose;
    $.verbose = false;
    const shaFile = path.join(await cwd(), `${outputDir}/.sha`);
    let previous = "";
    try {
        previous = (await fs.readFile(shaFile)).toString().trim();
    } catch {}
    const current = (await $`dirsh ${inputDirs}`).stdout.trim();
    $.verbose = verbosity;
    console.log({ previous, current });
    if (previous !== current) {
        await fn();
        await fs.writeFile(shaFile, current);
    }
}

export function getArch() {
    const operating_systems = {
        darwin: "macos",
        linux: "linux",
    };
    const architectures = {
        x64: "x86_64",
        arm64: "aarch64",
    };
    return `${architectures[os.arch()]}-${operating_systems[os.platform()]}`;
}

export async function getProject(dir) {
    const verbosity = $.verbose;
    $.verbose = false;
    const previous = await cwd();
    dir && cd(dir);
    const meta = JSON.parse(
        await $`cargo metadata --no-deps --format-version 1`
    );
    cd(previous);
    $.verbose = verbosity;
    return meta.packages[0];
}

export function step(msg) {
    console.log(chalk.blue.bold(`----\n${msg}`));
}

export function setColors() {
    process.env.CARGO_TERM_COLOR = "always";
    process.env.FORCE_COLOR = "3";
}

export async function cwd() {
    const verbosity = $.verbose;
    $.verbose = false;
    const cwd = (await $`pwd`).stdout.trim();
    $.verbose = verbosity;
    return cwd;
}
