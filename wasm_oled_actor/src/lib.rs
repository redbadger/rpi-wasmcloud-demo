#![allow(clippy::unnecessary_wraps)]

use actor_core as core;
use actor_http_server as http;
use oled_provider_interface as oled;
use wapc_guest::prelude::*;

#[no_mangle]
pub fn wapc_init() {
    core::Handlers::register_health_request(health);
    http::Handlers::register_handle_request(handler);
}

fn handler(payload: http::Request) -> HandlerResult<http::Response> {
    match payload.method.as_ref() {
        "POST" => {
            oled::default().update(String::from_utf8(payload.body)?)?;
            Ok(http::Response::ok())
        }
        "DELETE" => {
            oled::default().update(String::new())?;
            Ok(http::Response::ok())
        }
        _ => Ok(http::Response::bad_request()),
    }
}

fn health(_req: core::HealthCheckRequest) -> HandlerResult<core::HealthCheckResponse> {
    Ok(core::HealthCheckResponse::healthy())
}
