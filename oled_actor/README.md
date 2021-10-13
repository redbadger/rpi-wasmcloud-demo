# OLED Actor

This project implements an actor that sets or clears the text on the OLED provider.

Upon receiving an http POST request, the actor sends the body text to the provider.
Upon receiving an http DELETE request, the actor clears the OLED.
For example, if the http server is running on localhost port 8000,
the command

```bash
curl -D Hello "localhost:8000"
```

sets the OLED text to "Hello".

and

```bash
curl -X DELETE "localhost:8000"
```

clears the OLED text.

## The implementation

To respond to http requests, the actor must implement the
`httpResponse` method of the
[HttpServer interface](https://github.com/wasmCloud/interfaces/tree/main/httpserver) interface.

The implementation is in the file [src/lib.rs](./src/lib.rs)

## See it in action

- To compile the actor and generate a signed WebAssembly module, type `make`.
- To load and start the actor you'll need to have a running OCI-compatible
  registry. Check that `REG_URL` setting in Makefile is correct, and run
  `make push` and `make start` to push the actor to the registry
  and start the actor.
  Alternately, you can load and start the actor from the host's web ui.
  When prompted for the path,
  select `build/oled_actor_s.wasm`.

The actor must be linked with an HttpServer capability
provider with the contract id `wasmcloud:httpserver`. You can start the
provider (TODO: need registry url and more specific instructions here)
