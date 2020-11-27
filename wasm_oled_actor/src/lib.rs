#![allow(clippy::unnecessary_wraps)]

#[macro_use]
extern crate serde_json;

use actor::prelude::*;
use wascc_actor as actor;

actor_handlers! {
    codec::http::OP_HANDLE_REQUEST => handler,
    codec::core::OP_HEALTH_REQUEST => health
}

fn handler(payload: codec::http::Request) -> HandlerResult<codec::http::Response> {
    let res = untyped::default().call(
        "red-badger:oled-ssd1306",
        "Update",
        serialize(payload.path)?,
    )?;

    let result = json!(res);
    Ok(codec::http::Response::json(result, 200, "OK"))
}

fn health(_req: codec::core::HealthRequest) -> HandlerResult<()> {
    Ok(())
}
