use wascc_host::{Host, NativeCapability};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let _ = env_logger::try_init();
    let host = Host::new();
    host.add_native_capability(NativeCapability::from_file(
        "../oled-ssd1306/target/debug/liboled_ssd1306.so",
        None,
    )?)?;

    std::thread::park();

    Ok(())
}
