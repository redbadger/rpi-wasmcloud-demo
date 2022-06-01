package actor

import (
	"path"
	"dagger.io/dagger"
	"dagger.io/dagger/core"
	"universe.dagger.io/docker"
)

// build with cargo
#Build: {
	// source code
	sources: [srcPath=string]: dagger.#FS
	name:     "oled_actor"
	artefact: "\(name).wasm"

	_workDir: "/root"
	_outdir:  path.Join([_workDir, "actor/target/wasm32-unknown-unknown/release"], path.Unix)

	_run: docker.#Build & {
		steps: [
			docker.#Pull & {
				source: "rust:latest"
			},
			docker.#Run & {
				command: {
					name: "rustup"
					args: ["component", "add", "rustfmt"]
				}
			},
			docker.#Run & {
				command: {
					name: "rustup"
					args: ["target", "add", "wasm32-unknown-unknown"]
				}
			},
			for srcPath, source in sources {
				docker.#Copy & {
					dest:     path.Join([_workDir, srcPath], path.Unix)
					contents: source
				}
			},
			docker.#Run & {
				command: {
					name: "cargo"
					args: ["build", "--release"]
				}
				workdir: path.Join([_workDir, "actor"], path.Unix)
			},
		]
	}

	_subdir: core.#Subdir & {
		input: _run.output.rootfs
		path:  _outdir
	}
	output: dagger.#FS & _subdir.output
}
