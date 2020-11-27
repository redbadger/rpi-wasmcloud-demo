use wascc_host::{Host, NativeCapability};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let _ = env_logger::try_init();
    let host = Host::new();
    host.add_native_capability(NativeCapability::from_file(
        "../pi_oled_provider/target/debug/libpi_oled_provider.so",
        None,
    )?)?;

    std::thread::park();

    Ok(())
}
