package main

import (
	"dagger.io/dagger"

	"dev@red-badger.com/rpi-wasmcloud-demo/interface"
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
			"./interface/build": write: contents: actions.buildInterface.contents.output
			"./provider/build": write: contents:  actions.buildProvider.contents.output
			"./actor/build": write: contents:     actions.buildActor.contents.output
		}
		env: {}
	}
	actions: {
		buildInterface: interface.#Build & {
			source: client.filesystem."./interface".read.contents
		}
		buildProvider: provider.#Build & {
			sources: {
				interface: client.filesystem."./interface".read.contents
				provider:  client.filesystem."./provider".read.contents
			}
		}
		buildActor: actor.#Build & {
			sources: {
				interface: client.filesystem."./interface".read.contents
				actor:     client.filesystem."./actor".read.contents
			}
		}
	}
}
