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
	output:  dagger.#FS & _subdir.output
	_subdir: core.#Subdir & {
		input: _run.output.rootfs
		path:  _outdir
	}
	contents: core.#Copy & {
		input:    dagger.#Scratch
		contents: output
		include: ["oled-provider"]
	}
}
