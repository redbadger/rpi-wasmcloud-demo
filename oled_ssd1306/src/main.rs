use anyhow::Result;
use oled_ssd1306_interface::{Oled, OledReceiver, Request};
use wasmbus_rpc::provider::prelude::*;

#[cfg(target_os = "aarch64")]
mod display;
#[cfg(target_os = "aarch64")]
use anyhow::anyhow;
#[cfg(target_os = "aarch64")]
use display::say;
#[cfg(target_os = "aarch64")]
use log::debug;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    provider_main(OledProvider::default())?;

    eprintln!("oled provider exiting");
    Ok(())
}

#[derive(Default, Clone, Provider)]
#[services(Oled)]
struct OledProvider {}

/// use default implementations of provider message handlers
impl ProviderDispatch for OledProvider {}
impl ProviderHandler for OledProvider {}

/// Handle Oled methods
#[async_trait]
impl Oled for OledProvider {
    /// updates the text on the oled display
    #[cfg(target_os = "aarch64")]
    async fn update(&self, _ctx: &Context, req: &Request) -> RpcResult<()> {
        debug!("processing request update({})", req.text);
        say(&msg.txt).map_err(|e| anyhow!("error writing to display: {:?}", e))?;
        Ok(())
    }

    /// clears the oled display
    #[cfg(target_os = "aarch64")]
    async fn clear(&self, _ctx: &Context) -> RpcResult<()> {
        debug!("processing request clear()");
        say("").map_err(|e| anyhow!("error writing to display: {:?}", e))?;
        Ok(())
    }

    /// not implemented!
    #[cfg(not(target_os = "aarch64"))]
    async fn update(&self, _ctx: &Context, _req: &Request) -> RpcResult<()> {
        unimplemented!()
    }

    /// not implemented!
    #[cfg(not(target_os = "aarch64"))]
    async fn clear(&self, _ctx: &Context) -> RpcResult<()> {
        unimplemented!()
    }
}
