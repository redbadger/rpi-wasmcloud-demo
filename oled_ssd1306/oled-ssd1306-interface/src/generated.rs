extern crate log;
extern crate rmp_serde as rmps;

use rmps::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[cfg(feature = "guest")]
use lazy_static::lazy_static;
#[cfg(feature = "guest")]
use std::sync::RwLock;
#[cfg(feature = "guest")]
use wapc_guest::prelude::*;

#[cfg(feature = "guest")]
type UpdateResult = HandlerResult<UpdateResponse>;
#[cfg(feature = "guest")]
type UpdateFunction = fn(String) -> UpdateResult;
#[cfg(feature = "guest")]
type ClearFunction = fn() -> UpdateResult;

#[cfg(feature = "guest")]
pub struct Host {
    binding: String,
}

#[cfg(feature = "guest")]
impl Default for Host {
    fn default() -> Self {
        Host {
            binding: "default".to_string(),
        }
    }
}

/// Creates a named host binding
#[cfg(feature = "guest")]
pub fn host(binding: &str) -> Host {
    Host {
        binding: binding.to_string(),
    }
}

/// Creates the default host binding
#[cfg(feature = "guest")]
pub fn default() -> Host {
    Host::default()
}

#[cfg(feature = "guest")]
impl Host {
    pub fn update(&self, txt: String) -> UpdateResult {
        let input_args = UpdateArgs { txt };
        host_call(
            &self.binding,
            "red-badger:oled-ssd1306",
            "Update",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<UpdateResponse>(vec.as_ref()).unwrap();
            resp
        })
    }

    pub fn clear(&self) -> UpdateResult {
        let input_args = ClearArgs {};
        host_call(
            &self.binding,
            "red-badger:oled-ssd1306",
            "Clear",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<UpdateResponse>(vec.as_ref()).unwrap();
            resp
        })
    }
}

#[cfg(feature = "guest")]
pub struct Handlers {}

#[cfg(feature = "guest")]
impl Handlers {
    pub fn register_update(f: UpdateFunction) {
        *UPDATE.write().unwrap() = Some(f);
        register_function(&"Update", update_wrapper);
    }
    pub fn register_clear(f: ClearFunction) {
        *CLEAR.write().unwrap() = Some(f);
        register_function(&"Clear", clear_wrapper);
    }
}

#[cfg(feature = "guest")]
lazy_static! {
    static ref UPDATE: RwLock<Option<UpdateFunction>> = RwLock::new(None);
    static ref CLEAR: RwLock<Option<ClearFunction>> = RwLock::new(None);
}

#[cfg(feature = "guest")]
fn update_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<UpdateArgs>(input_payload)?;
    let lock = UPDATE.read().unwrap().unwrap();
    let result = lock(input.txt)?;
    serialize(result)
}

#[cfg(feature = "guest")]
fn clear_wrapper(input_payload: &[u8]) -> CallResult {
    let _input = deserialize::<ClearArgs>(input_payload)?;
    let lock = CLEAR.read().unwrap().unwrap();
    let result = lock()?;
    serialize(result)
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct UpdateArgs {
    #[serde(rename = "txt")]
    pub txt: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct ClearArgs {}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct UpdateResponse {
    #[serde(rename = "success")]
    pub success: bool,
}

/// The standard function for serializing codec structs into a format that can be
/// used for message exchange between actor and host. Use of any other function to
/// serialize could result in breaking incompatibilities.
pub fn serialize<T>(
    item: T,
) -> ::std::result::Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>
where
    T: Serialize,
{
    let mut buf = Vec::new();
    item.serialize(&mut Serializer::new(&mut buf).with_struct_map())?;
    Ok(buf)
}

/// The standard function for de-serializing codec structs from a format suitable
/// for message exchange between actor and host. Use of any other function to
/// deserialize could result in breaking incompatibilities.
pub fn deserialize<'de, T: Deserialize<'de>>(
    buf: &[u8],
) -> ::std::result::Result<T, Box<dyn std::error::Error + Send + Sync>> {
    let mut de = Deserializer::new(Cursor::new(buf));
    match Deserialize::deserialize(&mut de) {
        Ok(t) => Ok(t),
        Err(e) => Err(format!("Failed to de-serialize: {}", e).into()),
    }
}
