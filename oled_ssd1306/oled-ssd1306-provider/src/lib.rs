#[macro_use]
extern crate wascc_codec as codec;
#[macro_use]
extern crate log;

use anyhow::{anyhow, Result};
use codec::{
    capabilities::{CapabilityProvider, Dispatcher, NullDispatcher},
    core::{OP_BIND_ACTOR, OP_REMOVE_ACTOR},
};
use embedded_graphics::{
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    style::TextStyleBuilder,
};
use linux_embedded_hal::I2cdev;
use ssd1306::{prelude::*, Builder, I2CDIBuilder};
use std::{
    error::Error,
    sync::{Arc, RwLock},
};
use wasmcloud_actor_core::{deserialize, CapabilityConfiguration};

const SYSTEM_ACTOR: &str = "system";
#[allow(unused)]
const CAPABILITY_ID: &str = "red-badger:oled-ssd1306";

const OP_UPDATE: &str = "Update";
const OP_CLEAR: &str = "Clear";

#[cfg(not(feature = "static_plugin"))]
capability_provider!(OledSsd1306Provider, OledSsd1306Provider::new);

#[derive(Clone)]
pub struct OledSsd1306Provider {
    dispatcher: Arc<RwLock<Box<dyn Dispatcher>>>,
}

impl Default for OledSsd1306Provider {
    fn default() -> Self {
        let _ = env_logger::try_init();

        OledSsd1306Provider {
            dispatcher: Arc::new(RwLock::new(Box::new(NullDispatcher::new()))),
        }
    }
}

impl OledSsd1306Provider {
    pub fn new() -> Self {
        Self::default()
    }

    fn configure(
        &self,
        _config: CapabilityConfiguration,
    ) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        info!("configure!");
        Ok(vec![])
    }

    fn deconfigure(
        &self,
        _config: CapabilityConfiguration,
    ) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        info!("de-configure!");
        Ok(vec![])
    }

    fn clear(&self, _actor: &str) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        clear().map_err(|e| anyhow!("error writing to display: {:?}", e))?;
        Ok(vec![])
    }

    fn update(&self, _actor: &str, msg: String) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        say(&msg).map_err(|e| anyhow!("error writing to display: {:?}", e))?;
        Ok(vec![])
    }
}

impl CapabilityProvider for OledSsd1306Provider {
    // Invoked by the runtime host to give this provider plugin the ability to communicate
    // with actors
    fn configure_dispatch(
        &self,
        dispatcher: Box<dyn Dispatcher>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        info!("Dispatcher received.");
        let mut lock = self.dispatcher.write().unwrap();
        *lock = dispatcher;
        Ok(())
    }

    fn handle_call(
        &self,
        actor: &str,
        op: &str,
        msg: &[u8],
    ) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        info!("Handling operation `{}` from `{}`", op, actor);

        match op {
            OP_BIND_ACTOR if actor == SYSTEM_ACTOR => self.configure(deserialize(msg)?),
            OP_REMOVE_ACTOR if actor == SYSTEM_ACTOR => self.deconfigure(deserialize(msg)?),
            OP_CLEAR => self.clear(actor),
            OP_UPDATE => self.update(actor, deserialize(msg)?),
            _ => Err(format!("Unknown operation: {}", op).into()),
        }
    }

    fn stop(&self) {}
}

fn clear() -> Result<()> {
    let i2c = I2cdev::new("/dev/i2c-1")?;
    let interface = I2CDIBuilder::new().init(i2c);
    let mut display: GraphicsMode<_, _> = Builder::new().connect(interface).into();

    display
        .init()
        .map_err(|_| anyhow!("error initializing display"))?;

    display
        .flush()
        .map_err(|_| anyhow!("error flushing display"))
}

fn say(txt: &str) -> Result<()> {
    let i2c = I2cdev::new("/dev/i2c-1")?;
    let interface = I2CDIBuilder::new().init(i2c);
    let mut display: GraphicsMode<_, _> = Builder::new().connect(interface).into();

    display
        .init()
        .map_err(|_| anyhow!("error initializing display"))?;

    let text_style = TextStyleBuilder::new(Font6x8)
        .text_color(BinaryColor::On)
        .build();

    Text::new(txt, Point::new(0, 0))
        .into_styled(text_style)
        .draw(&mut display)
        .map_err(|_| anyhow!("error writing text"))?;

    display
        .flush()
        .map_err(|_| anyhow!("error flushing display"))
}
