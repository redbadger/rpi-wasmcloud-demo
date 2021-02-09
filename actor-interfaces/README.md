# wasmCloud Capability Providers

Install `wapc`:

```sh
npm install -g git+https://github.com/wapc/cli.git#master
```

## Create a new capability

For example for the `oled_ssd1306` provider:

```sh
wapc new rust oled_ssd1306
```

Edit the `schema.widl` file, then `(cd oled_ssd1306 && make)`.
