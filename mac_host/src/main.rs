use std::collections::HashMap;
use wascc_host::{Actor, Host, NativeCapability};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let _ = env_logger::try_init();
    let host = Host::new();
    host.add_actor(Actor::from_file(
        "../wasm_oled_actor/target/wasm32-unknown-unknown/debug/wasm_oled_actor_signed.wasm",
    )?)?;
    host.add_native_capability(NativeCapability::from_file(
        "../../../src/capability-providers/http-server/target/debug/libwascc_httpsrv.dylib",
        None,
    )?)?;

    host.set_binding(
        "MDNKUUYGVK2OSEH5EUBRF2UYABTTYZFZCF6WA5OF7GURJFB235CLUZMN",
        "wascc:http_server",
        None,
        generate_port_config(8081),
    )?;
    host.set_binding(
        "MDNKUUYGVK2OSEH5EUBRF2UYABTTYZFZCF6WA5OF7GURJFB235CLUZMN",
        "red-badger:oled-ssd1306",
        None,
        HashMap::new(),
    )?;

    std::thread::park();

    Ok(())
}

fn generate_port_config(port: u16) -> HashMap<String, String> {
    let mut hm = HashMap::new();
    hm.insert("PORT".to_string(), port.to_string());

    hm
}
