#![allow(clippy::unnecessary_wraps)]

use log::info;
use wapc_guest::HandlerResult;
use wasmcloud_actor_core as core;
use wasmcloud_actor_http_server as http;
use wasmcloud_actor_logging as logging;

#[no_mangle]
pub fn wapc_init() {
    http::Handlers::register_handle_request(handler);
    core::Handlers::register_health_request(health);
    logging::enable_macros();
}

fn handler(request: http::Request) -> HandlerResult<http::Response> {
    match request.method.as_ref() {
        "POST" => {
            let txt = String::from_utf8(request.body)?;
            info!("updating display with: {}", txt);
            oled_ssd1306_interface::default()
                .update(txt)
                .map(|_| http::Response::ok())
                .or_else(|e| {
                    info!("error updating display: {:?}", e);
                    Ok(http::Response::internal_server_error(
                        "There was a problem updating the display",
                    ))
                })
        }
        "DELETE" => {
            info!("clearing display");
            oled_ssd1306_interface::default()
                .clear()
                .map(|_| http::Response::ok())
                .or_else(|e| {
                    info!("error clearing display: {:?}", e);
                    Ok(http::Response::internal_server_error(
                        "There was a problem clearing the display",
                    ))
                })
        }
        _ => Ok(http::Response::bad_request()),
    }
}

fn health(_request: core::HealthCheckRequest) -> HandlerResult<core::HealthCheckResponse> {
    Ok(core::HealthCheckResponse::healthy())
}
