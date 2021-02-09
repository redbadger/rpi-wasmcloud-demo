# wasmCloud Capability Providers

Install `wapc`:

```sh
npm install -g git+https://github.com/wapc/cli.git#master
```

## Create a new capability

For example for the `oled` provider:

```sh
wapc new rust oled
```

Edit the `schema.widl` file, then `(cd oled && make)`.
