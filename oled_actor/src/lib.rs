#![allow(clippy::unnecessary_wraps)]

use wapc_guest::prelude::*;
use wasmcloud_actor_core as core;
use wasmcloud_actor_http_server as http;

#[no_mangle]
pub fn wapc_init() {
    core::Handlers::register_health_request(health);
    http::Handlers::register_handle_request(handler);
}

fn handler(payload: http::Request) -> HandlerResult<http::Response> {
    match payload.method.as_ref() {
        "POST" => {
            oled_ssd1306_interface::default().update(String::from_utf8(payload.body)?)?;
            Ok(http::Response::ok())
        }
        "DELETE" => {
            oled_ssd1306_interface::default().clear()?;
            Ok(http::Response::ok())
        }
        _ => Ok(http::Response::bad_request()),
    }
}

fn health(_req: core::HealthCheckRequest) -> HandlerResult<core::HealthCheckResponse> {
    Ok(core::HealthCheckResponse::healthy())
}
