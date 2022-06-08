package provider

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

	_workDir: "/root"
	_outdir:  path.Join([_workDir, "/provider/target/release"], path.Unix)
	artefact: "oled-provider"

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
				workdir: path.Join([_workDir, "provider"], path.Unix)
			},
		]
	}

	_subdir: core.#Subdir & {
		input: _run.output.rootfs
		path:  _outdir
	}
	output: _subdir.output
}
