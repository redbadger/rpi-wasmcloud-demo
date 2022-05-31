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

	_buildCachePath: "/root/target"
	_outdir:         "/root/actor/target/wasm32-unknown-unknown/release"

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
					dest:     "/root/" + path
					contents: source
				}
			},
			docker.#Run & {
				mounts: {
					"build cache": {
						dest:     _buildCachePath
						type:     "cache"
						contents: core.#CacheDir & {
							id: "actor-build-cache"
						}
					}
				}
				command: {
					name: "cargo"
					args: ["build", "--release"]
				}
				workdir: "/root/actor"
			},
		]
	}
	output:  dagger.#FS & _subdir.output
	_subdir: core.#Subdir & {
		input: _run.output.rootfs
		path:  _outdir
	}
	contents: core.#Copy & {
		input:    dagger.#Scratch
		contents: output
		include: ["oled_actor.wasm"]
	}
}
