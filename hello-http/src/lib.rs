#[macro_use]
extern crate serde_json;

use actor::prelude::*;
use wascc_actor as actor;

actor_handlers! {
    codec::http::OP_HANDLE_REQUEST => hello_world,
    codec::core::OP_HEALTH_REQUEST => health
}

fn hello_world(_payload: codec::http::Request) -> HandlerResult<codec::http::Response> {
    let result = json!({ "hello": "world", "data": 21});
    Ok(codec::http::Response::json(result, 200, "OK"))
}

fn health(_req: codec::core::HealthRequest) -> HandlerResult<()> {
    Ok(())
}
