mod routes;
mod websocket;

use crate::config::Config;
use axum::{
    Router,
};
use tokio::net::TcpListener;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};

pub async fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .merge(routes::routes())
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    let address = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::bind(&address).await?;

    tracing::info!("FreeJarvisV2 listening on {address}");

    axum::serve(listener, app).await?;

    Ok(())
}
