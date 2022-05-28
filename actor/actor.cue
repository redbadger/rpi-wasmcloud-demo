package actor

import (
	"dagger.io/dagger"
	"dagger.io/dagger/core"
	"universe.dagger.io/docker"
)

// build with cargo
#Build: {
	// source code
	sources: [path=string]: dagger.#FS

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
			for path, source in sources {
				docker.#Copy & {
					dest:     "/src/" + path
					contents: source
				}
			},

			docker.#Run & {
				command: {
					name: "cargo"
					args: ["build"]
				}
				workdir: "/src/actor"
			},
		]
	}
	contents: core.#Copy & {
		input:    dagger.#Scratch
		contents: _run.output.rootfs
		source:   "/src/actor/target/wasm32-unknown-unknown/debug/*"
		include: ["oled_actor.wasm"]
	}
}
