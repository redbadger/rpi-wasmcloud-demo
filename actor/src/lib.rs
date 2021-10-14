use oled_ssd1306_interface::{Oled, OledSender, Request};
use once_cell::sync::OnceCell;
use serde::Deserialize;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_logging::{info, warn};
use wasmcloud_interface_numbergen::{NumberGen, NumberGenSender};

static ID: OnceCell<String> = OnceCell::new();

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct OledActor {}

#[async_trait]
impl HttpServer for OledActor {
    async fn handle_request(
        &self,
        ctx: &Context,
        req: &HttpRequest,
    ) -> std::result::Result<HttpResponse, RpcError> {
        let id = match ID.get() {
            Some(id) => id.to_owned(),
            None => NumberGenSender::new().generate_guid(ctx).await?,
        };
        let trimmed_path = match req.path.trim_end_matches('/') {
            "" => "/",
            x => x,
        };

        match (req.method.as_ref(), trimmed_path) {
            ("POST", "/") => {
                let text = deser(&req.body)?;
                info!("{}: updating display with: {}", id, text);
                OledSender::new().update(ctx, &Request { text }).await?;
                Ok(HttpResponse::default())
            }
            ("DELETE", "/") => {
                info!("{}: clearing display", id);
                OledSender::new().clear(ctx).await?;
                Ok(HttpResponse::default())
            }
            (_, _) => {
                warn!("no route for this request: {:?}", req);
                Ok(HttpResponse::not_found())
            }
        }
    }
}

fn deser<'de, T: Deserialize<'de>>(raw: &'de [u8]) -> RpcResult<T> {
    serde_json::from_slice(raw).map_err(|e| RpcError::Deser(format!("{}", e)))
}
