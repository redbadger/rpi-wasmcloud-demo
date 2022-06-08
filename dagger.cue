package main

import (
	"dagger.io/dagger"
	"dagger.io/dagger/core"
	"dev@red-badger.com/rpi-wasmcloud-demo/provider"
	"dev@red-badger.com/rpi-wasmcloud-demo/actor"
)

dagger.#Plan & {
	client: {
		filesystem: {
			"./interface": read: {
				contents: dagger.#FS
				exclude: ["rust/target"]
			}
			"./provider": read: {
				contents: dagger.#FS
				exclude: ["target"]
			}
			"./actor": read: {
				contents: dagger.#FS
				exclude: ["target"]
			}

			"./build/provider": write: contents: actions.buildProvider.output
			"./build/actor": write: contents:    actions.buildActor.output
		}
		env: {}
	}
	actions: {
		_interface_src: client.filesystem."./interface".read.contents

		_buildProvider: provider.#Build & {
			sources: {
				interface: _interface_src
				provider:  client.filesystem."./provider".read.contents
			}
		}
		buildProvider: core.#Copy & {
			input:    dagger.#Scratch
			contents: _buildProvider.output
			include: [_buildProvider.artefact]
		}

		_buildActor: actor.#Build & {
			sources: {
				interface: _interface_src
				actor:     client.filesystem."./actor".read.contents
			}
		}
		_signActor: actor.#Sign & {
			source: _buildActor.output
		}
		buildActor: core.#Copy & {
			input:    dagger.#Scratch
			contents: _signActor.output
			include: [_buildActor.artefact, _signActor.artefact]
		}
	}
}
