package provider

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
			for path, source in sources {
				docker.#Copy & {
					dest:     "/src/" + path
					contents: source
				}
			},

			docker.#Run & {
				command: {
					name: "cargo"
					args: ["build", "--release"]
				}
				workdir: "/src/provider"
			},
		]
	}
	contents: core.#Copy & {
		input:    dagger.#Scratch
		contents: _run.output.rootfs
		source:   "/src/provider/target/release/*"
		include: ["provider"]
	}
}
