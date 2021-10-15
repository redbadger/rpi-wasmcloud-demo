use oled_ssd1306_interface::{Oled, OledSender, Request};
use once_cell::sync::OnceCell;
use std::str;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_logging::{info, warn};
use wasmcloud_interface_numbergen::{NumberGen, NumberGenSender};

static INSTANCE: OnceCell<String> = OnceCell::new();

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
        let instance = match INSTANCE.get() {
            Some(instance) => instance.to_owned(),
            None => {
                let instance = NumberGenSender::new().generate_guid(ctx).await?;
                INSTANCE.set(instance.clone())?;
                instance
            }
        };
        let trimmed_path = match req.path.trim_end_matches('/') {
            "" => "/",
            x => x,
        };

        match (req.method.as_ref(), trimmed_path) {
            ("POST", "/") => {
                let text = str::from_utf8(&req.body)
                    .map_err(|e| RpcError::Deser(format!("{}", e)))?
                    .to_string();
                info!("{}: updating display with: {}", instance, text);
                OledSender::new().update(ctx, &Request { text }).await?;
                Ok(HttpResponse::default())
            }
            ("DELETE", "/") => {
                info!("{}: clearing display", instance);
                OledSender::new().clear(ctx).await?;
                Ok(HttpResponse::default())
            }
            (_, _) => {
                warn!("{}: no route for this request: {:?}", instance, req);
                Ok(HttpResponse::not_found())
            }
        }
    }
}
