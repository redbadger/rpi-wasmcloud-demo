use display::say;
use log::info;
use oled_interface::{Oled, OledReceiver, Request};
use wasmbus_rpc::provider::prelude::*;

mod display;
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
    async fn update(&self, _ctx: &Context, req: &Request) -> RpcResult<()> {
        info!("processing request update({})", req.text);
        say(&req.text)
            .map_err(|e| RpcError::Other(format!("error writing to display: {:?}", e)))?;
        Ok(())
    }

    /// clears the oled display
    async fn clear(&self, _ctx: &Context) -> RpcResult<()> {
        info!("processing request clear()");
        say("").map_err(|e| RpcError::Other(format!("error writing to display: {:?}", e)))?;
        Ok(())
    }
}
