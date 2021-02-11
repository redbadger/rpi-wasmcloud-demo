# wasmCloud Capability Providers

Install `wapc`:

```sh
npm install -g git+https://github.com/wapc/cli.git#master
```

## Create a new capability

For example for the `oled-ssd1306-interface` provider:

```sh
wapc new rust oled-ssd1306-interface
```

Edit the `schema.widl` file, then `(cd oled-ssd1306-interface && make interface)`.
