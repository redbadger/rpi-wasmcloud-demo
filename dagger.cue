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
				exclude: ["rust/target", "build", "node_modules", ".turbo"]
			}
			"./provider": read: {
				contents: dagger.#FS
				exclude: ["target", "build", "node_modules", ".turbo"]
			}
			"./actor": read: {
				contents: dagger.#FS
				exclude: ["target", "build", "node_modules", ".turbo"]
			}

			"./provider/build": write: contents: actions.buildProvider.contents.output

			_out: core.#Merge & {
				inputs: [
					if (actions.buildActor.contents.output != _|_) {
						actions.buildActor.contents.output
					},
					if (actions.buildSignedActor.contents.output != _|_) {
						actions.buildSignedActor.contents.output
					},
				]
			}
			"./actor/build": write: contents: _out.output
		}
		env: {}
	}
	actions: {
		_interface_src: client.filesystem."./interface".read.contents

		buildProvider: provider.#Build & {
			sources: {
				interface: _interface_src
				provider:  client.filesystem."./provider".read.contents
			}
		}
		buildActor: actor.#Build & {
			sources: {
				interface: _interface_src
				actor:     client.filesystem."./actor".read.contents
			}
		}
		buildSignedActor: actor.#Sign & {
			source: buildActor.output
		}
	}
}
