use std::net::SocketAddr;

use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

mod auth;
mod config;
mod metrics;
mod router;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Setup metrics and config
    metrics::init();
    let settings = config::load();

    // Build Axum app
    let app = router::create_router(settings)
        .layer(TraceLayer::new_for_http());

    // Bind TCP listener
    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    let listener = TcpListener::bind(addr).await.unwrap();

    tracing::info!("🚀 RustGate listening on http://{}", addr);

    // Start server using Axum's `serve`
    axum::serve(listener, app).await.unwrap();
}
