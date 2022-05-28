package main

import (
	"dagger.io/dagger"

	// "dev@red-badger.com/rpi-wasmcloud-demo/interface"
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
			"./build/provider": write: contents: actions.buildProvider.contents.output
			"./build/actor": write: contents:    actions.buildActor.contents.output
		}
		env: {}
	}
	actions: {
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
