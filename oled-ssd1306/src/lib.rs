#[macro_use]
extern crate wascc_codec as codec;

#[macro_use]
extern crate log;

use codec::{
    capabilities::{
        CapabilityDescriptor, CapabilityProvider, Dispatcher, NullDispatcher, OperationDirection,
        OP_GET_CAPABILITY_DESCRIPTOR,
    },
    core::{CapabilityConfiguration, OP_BIND_ACTOR, OP_REMOVE_ACTOR},
    deserialize, serialize,
};
use std::{error::Error, sync::RwLock};

const SYSTEM_ACTOR: &str = "system";
const CAPABILITY_ID: &str = "red-badger:oled-ssd1306"; // TODO: change this to an appropriate capability ID
const VERSION: &str = env!("CARGO_PKG_VERSION");
const REVISION: u32 = 0; // Typically incremented after each publication in crates or [gantry](https://github.com/wascc/gantry)

const OP_UPDATE: &str = "Update";

#[cfg(not(feature = "static_plugin"))]
capability_provider!(OledSsd1306Provider, OledSsd1306Provider::new);

pub struct OledSsd1306Provider {
    dispatcher: RwLock<Box<dyn Dispatcher>>,
}

impl Default for OledSsd1306Provider {
    fn default() -> Self {
        let _ = env_logger::try_init();

        OledSsd1306Provider {
            dispatcher: RwLock::new(Box::new(NullDispatcher::new())),
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
    ) -> Result<Vec<u8>, Box<dyn Error + Sync + Send>> {
        // Handle actor binding metadata here...
        // This is typically where you would establish a
        // client or connection to a resource on behalf of
        // an actor
        println!("configure!");

        Ok(vec![])
    }

    fn deconfigure(
        &self,
        _config: CapabilityConfiguration,
    ) -> Result<Vec<u8>, Box<dyn Error + Sync + Send>> {
        // Handle removal of resources claimed by an actor here
        println!("de-configure!");

        Ok(vec![])
    }

    // Capability providers must provide a descriptor to the host containing metadata and a list of supported operations
    fn get_descriptor(&self) -> Result<Vec<u8>, Box<dyn Error + Sync + Send>> {
        Ok(serialize(
            CapabilityDescriptor::builder()
                .id(CAPABILITY_ID)
                .name("Red Badger OledSsd1306 Capability Provider") // TODO: change this friendly name
                .long_description("An OLED SSD1306 capability provider for waSCC actors") // TODO: change this documentation
                .version(VERSION)
                .revision(REVISION)
                .with_operation(
                    OP_UPDATE,
                    OperationDirection::Both,
                    "Updates text on the OLED display",
                ) // TODO: make the operation descriptors match your real interface
                .build(),
        )?)
    }

    fn update(&self, _actor: &str, txt: String) -> Result<Vec<u8>, Box<dyn Error + Sync + Send>> {
        println!("update {}", txt);
        todo!()
    }
}

impl CapabilityProvider for OledSsd1306Provider {
    // Invoked by the runtime host to give this provider plugin the ability to communicate
    // with actors
    fn configure_dispatch(
        &self,
        dispatcher: Box<dyn Dispatcher>,
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        trace!("Dispatcher received.");
        let mut lock = self.dispatcher.write().unwrap();
        *lock = dispatcher;

        Ok(())
    }

    // Invoked by host runtime to allow an actor to make use of the capability
    // All providers MUST handle the "configure" message, even if no work will be done
    fn handle_call(
        &self,
        actor: &str,
        op: &str,
        msg: &[u8],
    ) -> Result<Vec<u8>, Box<dyn Error + Sync + Send>> {
        trace!("Received host call from {}, operation - {}", actor, op);

        match op {
            OP_BIND_ACTOR if actor == SYSTEM_ACTOR => self.configure(deserialize(msg)?),
            OP_REMOVE_ACTOR if actor == SYSTEM_ACTOR => self.deconfigure(deserialize(msg)?),
            OP_GET_CAPABILITY_DESCRIPTOR if actor == SYSTEM_ACTOR => self.get_descriptor(),
            OP_UPDATE => self.update(actor, deserialize(msg)?),
            _ => Err("bad dispatch".into()),
        }
    }
}
