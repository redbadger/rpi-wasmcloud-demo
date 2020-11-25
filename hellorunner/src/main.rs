use std::collections::HashMap;
use wascc_host::{Actor, Host, NativeCapability};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let _ = env_logger::try_init();
    let host = Host::new();
    host.add_actor(Actor::from_file(
        "../hello-http/target/wasm32-unknown-unknown/debug/hello_http_signed.wasm",
    )?)?;
    host.add_native_capability(NativeCapability::from_file(
        "../../../src/capability-providers/http-server/target/debug/libwascc_httpsrv.dylib",
        None,
    )?)?;

    host.set_binding(
        "MAKXA4O2L3ZI3SBADO3XVSPVB5MZ4HQBODVTRWKRCJSUQ5KVB6S3TSDW",
        "wascc:http_server",
        None,
        generate_port_config(8081),
    )?;

    std::thread::park();

    Ok(())
}

fn generate_port_config(port: u16) -> HashMap<String, String> {
    let mut hm = HashMap::new();
    hm.insert("PORT".to_string(), port.to_string());

    hm
}
