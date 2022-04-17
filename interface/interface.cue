package interface

import (
	"dagger.io/dagger"
	"dagger.io/dagger/core"
	"universe.dagger.io/docker"
)

// build with cargo
#Build: {
	// source code
	source: dagger.#FS

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

			docker.#Copy & {
				dest:     "/tmp"
				contents: source
			},

			docker.#Run & {
				command: {
					name: "cargo"
					args: ["build"]
				}
				workdir: "/tmp/rust"
			},
		]
	}
	contents: core.#Copy & {
		input:    dagger.#Scratch
		contents: _run.output.rootfs
		source:   "/tmp/rust/target/*"
		include: ["liboled_interface.*"]
	}
}

dagger.#Plan & {
	client: {
		filesystem: {
			".": read: {
				contents: dagger.#FS
				exclude: [
					"build",
					"interface.cue",
					"rust/target",
				]
			}
			"./build": write: contents: actions.build.contents.output
		}
		env: {}
	}
	actions: {
		build: #Build & {
			source: client.filesystem.".".read.contents
		}
	}
}
