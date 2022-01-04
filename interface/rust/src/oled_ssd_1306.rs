// This file is generated automatically using wasmcloud/weld-codegen and smithy model definitions
//

#![allow(unused_imports, clippy::ptr_arg, clippy::needless_lifetimes)]
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, io::Write, string::ToString};
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
                let _resp = Oled::update(self, ctx, &value).await?;
                let buf = Vec::new();
                Ok(Message {
                    method: "Oled.Update",
                    arg: Cow::Owned(buf),
                })
            }
            "Clear" => {
                let _resp = Oled::clear(self, ctx).await?;
                let buf = Vec::new();
                Ok(Message {
                    method: "Oled.Clear",
                    arg: Cow::Owned(buf),
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

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
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
        let buf = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Oled.Update",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        Ok(())
    }
    #[allow(unused)]
    async fn clear(&self, ctx: &Context) -> RpcResult<()> {
        let buf = *b"";
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Oled.Clear",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        Ok(())
    }
}
