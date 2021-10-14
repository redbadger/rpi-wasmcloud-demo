// This file is generated automatically using wasmcloud-weld and smithy model definitions
//

#![allow(clippy::ptr_arg)]
#[allow(unused_imports)]
use async_trait::async_trait;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::{borrow::Cow, string::ToString};
#[allow(unused_imports)]
use wasmbus_rpc::{
    deserialize, serialize, Context, Message, MessageDispatch, RpcError, RpcResult, SendOpts,
    Timestamp, Transport,
};

pub const SMITHY_VERSION: &str = "1.0";

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Request {
    #[serde(default)]
    pub text: String,
}

/// wasmbus.contractId: red-badger:oled-ssd1306
/// wasmbus.providerReceive
/// wasmbus.actorReceive
#[async_trait]
pub trait Oled {
    /// returns the capability contract id for this interface
    fn contract_id() -> &'static str {
        "red-badger:oled-ssd1306"
    }
    async fn update(&self, ctx: &Context, arg: &Request) -> RpcResult<()>;
    async fn clear(&self, ctx: &Context) -> RpcResult<()>;
}

/// OledReceiver receives messages defined in the Oled service trait
#[doc(hidden)]
#[async_trait]
pub trait OledReceiver: MessageDispatch + Oled {
    async fn dispatch(&self, ctx: &Context, message: &Message<'_>) -> RpcResult<Message<'_>> {
        match message.method {
            "Update" => {
                let value: Request = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = Oled::update(self, ctx, &value).await?;
                let buf = Cow::Owned(serialize(&resp)?);
                Ok(Message {
                    method: "Oled.Update",
                    arg: buf,
                })
            }
            "Clear" => {
                let resp = Oled::clear(self, ctx).await?;
                let buf = Cow::Owned(serialize(&resp)?);
                Ok(Message {
                    method: "Oled.Clear",
                    arg: buf,
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "Oled::{}",
                message.method
            ))),
        }
    }
}

/// OledSender sends messages to a Oled service
/// client for sending Oled messages
#[derive(Debug)]
pub struct OledSender<T: Transport> {
    transport: T,
}

impl<T: Transport> OledSender<T> {
    /// Constructs a OledSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'send> OledSender<wasmbus_rpc::provider::ProviderTransport<'send>> {
    /// Constructs a Sender using an actor's LinkDefinition,
    /// Uses the provider's HostBridge for rpc
    pub fn for_actor(ld: &'send wasmbus_rpc::core::LinkDefinition) -> Self {
        Self {
            transport: wasmbus_rpc::provider::ProviderTransport::new(ld, None),
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl OledSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for actor-to-actor messaging
    /// using the recipient actor's public key
    pub fn to_actor(actor_id: &str) -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_actor(actor_id.to_string()).unwrap();
        Self { transport }
    }
}

#[cfg(target_arch = "wasm32")]
impl OledSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for sending to a Oled provider
    /// implementing the 'red-badger:oled-ssd1306' capability contract, with the "default" link
    pub fn new() -> Self {
        let transport = wasmbus_rpc::actor::prelude::WasmHost::to_provider(
            "red-badger:oled-ssd1306",
            "default",
        )
        .unwrap();
        Self { transport }
    }

    /// Constructs a client for sending to a Oled provider
    /// implementing the 'red-badger:oled-ssd1306' capability contract, with the specified link name
    pub fn new_with_link(link_name: &str) -> wasmbus_rpc::RpcResult<Self> {
        let transport = wasmbus_rpc::actor::prelude::WasmHost::to_provider(
            "red-badger:oled-ssd1306",
            link_name,
        )?;
        Ok(Self { transport })
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> Oled for OledSender<T> {
    #[allow(unused)]
    async fn update(&self, ctx: &Context, arg: &Request) -> RpcResult<()> {
        let arg = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Oled.Update",
                    arg: Cow::Borrowed(&arg),
                },
                None,
            )
            .await?;
        Ok(())
    }
    #[allow(unused)]
    async fn clear(&self, ctx: &Context) -> RpcResult<()> {
        let arg = *b"";
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Oled.Clear",
                    arg: Cow::Borrowed(&arg),
                },
                None,
            )
            .await?;
        Ok(())
    }
}
