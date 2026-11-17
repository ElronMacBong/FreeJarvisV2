mod routes;
mod websocket;

use crate::config::Config;
use tokio::net::TcpListener;

pub async fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let app = routes::routes();

    let address = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::bind(&address).await?;

    tracing::info!("FreeJarvisV2 listening on {address}");

    axum::serve(listener, app).await?;

    Ok(())
}
