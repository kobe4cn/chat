use anyhow::Result;

use notify_server::get_router;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};
#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().pretty().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();
    let addr = "0.0.0.0:6687";
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("Server is listening on {})", addr);
    let route = get_router();
    axum::serve(listener, route.into_make_service()).await?;

    Ok(())
}
