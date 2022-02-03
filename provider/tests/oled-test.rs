use oled_interface::*;
use wasmbus_rpc::{provider::prelude::Context, RpcResult};
use wasmcloud_test_util::{
    check,
    cli::print_test_results,
    provider_test::test_provider,
    run_selected_spawn,
    testing::{TestOptions, TestResult},
};

#[tokio::test]
async fn run_all() {
    let opts = TestOptions::default();
    let res = run_selected_spawn!(&opts, health_check, update, clear);
    print_test_results(&res);

    let passed = res.iter().filter(|tr| tr.passed).count();
    let total = res.len();
    assert_eq!(passed, total, "{} passed out of {}", passed, total);

    // try to let the provider shut down gracefully
    let provider = test_provider().await;
    let _ = provider.shutdown().await;
}

async fn health_check(_opt: &TestOptions) -> RpcResult<()> {
    let prov = test_provider().await;
    let hc = prov.health_check().await;

    check!(hc.is_ok())?;
    Ok(())
}

async fn update(_opt: &TestOptions) -> RpcResult<()> {
    let prov = test_provider().await;
    let oled = OledSender::via(prov);
    let ctx = Context::default();

    oled.update(
        &ctx,
        &Request {
            text: "hello".to_string(),
        },
    )
    .await?;

    Ok(())
}

async fn clear(_opt: &TestOptions) -> RpcResult<()> {
    let prov = test_provider().await;
    let oled = OledSender::via(prov);
    let ctx = Context::default();

    oled.clear(&ctx).await?;

    Ok(())
}
