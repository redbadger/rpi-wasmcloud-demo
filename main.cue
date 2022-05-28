package main

import (
	"dagger.io/dagger"

	// "dev@red-badger.com/rpi-wasmcloud-demo/interface"
	"dev@red-badger.com/rpi-wasmcloud-demo/provider"
)

dagger.#Plan & {
	client: {
		filesystem: {
			"./interface": read: {
				contents: dagger.#FS
				exclude: [
					"build",
					"rust/target",
				]
			}
			"./provider": read: {
				contents: dagger.#FS
				exclude: [
					"build",
					"target",
				]
			}
			"./build": write: contents: actions.build.contents.output
		}
		env: {}
	}
	actions: {
		build: provider.#Build & {
			sources: {
				interface: client.filesystem."./interface".read.contents
				provider:  client.filesystem."./provider".read.contents
			}
		}
	}
}
