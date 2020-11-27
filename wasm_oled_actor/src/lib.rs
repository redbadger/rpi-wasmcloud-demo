#![allow(clippy::unnecessary_wraps)]

#[macro_use]
extern crate serde_json;

use actor::prelude::*;
use codec::{
    core::{self, HealthRequest},
    http::{self, Request, Response},
};
use wascc_actor as actor;

const CAP_OLED: &str = "red-badger:oled-ssd1306";

actor_handlers! {
    http::OP_HANDLE_REQUEST => handler,
    core::OP_HEALTH_REQUEST => health
}

fn handler(payload: Request) -> HandlerResult<Response> {
    match payload.method.as_ref() {
        "POST" => {
            let body = std::str::from_utf8(payload.body.as_slice())?;
            let res = untyped::default().call(CAP_OLED, "Update", serialize(body)?)?;

            let result = json!(res);
            Ok(Response::json(result, 200, "OK"))
        }
        "DELETE" => {
            untyped::default().call(CAP_OLED, "Clear", vec![])?;
            Ok(Response::ok())
        }
        _ => Ok(Response::bad_request()),
    }
}

fn health(_req: HealthRequest) -> HandlerResult<()> {
    Ok(())
}
