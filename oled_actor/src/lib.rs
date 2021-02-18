#![allow(clippy::unnecessary_wraps)]

use log::info;
use serde_json::json;
use wapc_guest::prelude::*;
use wasmcloud_actor_core as core;
use wasmcloud_actor_http_server as http;
use wasmcloud_actor_logging as logging;

#[no_mangle]
pub fn wapc_init() {
    core::Handlers::register_health_request(health);
    http::Handlers::register_handle_request(handler);
    logging::enable_macros();
}

fn handler(payload: http::Request) -> HandlerResult<http::Response> {
    match payload.method.as_ref() {
        "POST" => {
            let txt = String::from_utf8(payload.body)?;
            info!("received text: {}", txt);
            match oled_ssd1306_interface::default().update(txt) {
                Ok(_) => Ok(http::Response::ok()),
                Err(e) => {
                    info!("update display error: {:?}", e);
                    let result = json!({ "error": e.to_string() });
                    Ok(http::Response::json(&result, 500, "Server Error"))
                }
            }
        }
        "DELETE" => match oled_ssd1306_interface::default().clear() {
            Ok(_) => Ok(http::Response::ok()),
            Err(e) => {
                info!("update display error: {:?}", e);
                Err(e)
            }
        },
        _ => Ok(http::Response::bad_request()),
    }
}

fn health(_req: core::HealthCheckRequest) -> HandlerResult<core::HealthCheckResponse> {
    Ok(core::HealthCheckResponse::healthy())
}
