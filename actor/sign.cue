package actor

import (
	"list"
	"dagger.io/dagger"
	"dagger.io/dagger/core"
	"universe.dagger.io/docker"
)

// sign with wash
#Sign: {
	source:   dagger.#FS
	name:     "oled_actor"
	version:  string | *"0.1"
	revision: string | *"0"
	claims: [
		"redbadger:oled",
		"wasmcloud:builtin:logging",
		"wasmcloud:builtin:numbergen",
		"wasmcloud:httpserver",
	]
	inFile:  string | *"\(name).wasm"
	outFile: string | *"\(name)_s.wasm"

	_workDir: "/root"

	_image: docker.#Pull & {
		source: "wasmcloud/wash"
	}

	_copy: docker.#Copy & {
		input:    _image.output
		contents: source
		dest:     _workDir
	}

	_#wash: docker.#Run & {
		input:   _copy.output
		user:    "root"
		workdir: _workDir
	}

	sign: _#wash & {
		command: {
			name: "claims"
			args: [ "sign",
				"--name", name,
				"--ver", version,
				"--rev", revision,
				"--destination", outFile,
			] + list.FlattenN([ for cap in claims {
				["--cap", cap]
			}], 1) + [inFile]
		}
	}

	_subdir: core.#Subdir & {
		input: sign.output.rootfs
		path:  _workDir
	}
	output: dagger.#FS & _subdir.output

	contents: core.#Copy & {
		input:    dagger.#Scratch
		contents: output
		include: [outFile]
	}
}
