use anyhow::Result;

use notify_server::{get_router, setup_pg_listener, AppConfig};
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};
#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().pretty().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();
    let config = AppConfig::try_load()?;
    let (route, state) = get_router(config.clone());

    let addr = format!("{}:{}", config.server.host, config.server.port);
    setup_pg_listener(state).await?;
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("Server is listening on {})", addr);

    axum::serve(listener, route.into_make_service()).await?;

    Ok(())
}
