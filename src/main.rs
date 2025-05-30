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

    // Start metrics server
    metrics::init();

    // Load configuration
    let settings = config::load();
    tracing::info!("✅ Loaded config: {:?}", settings);

    // Build the Axum app
    let app = router::create_router(settings.clone())
        .layer(TraceLayer::new_for_http());

    // Construct bind address from config
    let addr: SocketAddr = format!(
        "{}:{}",
        settings.server.address,
        settings.server.port
    )
        .parse()
        .expect("Invalid address in config");

    let listener = TcpListener::bind(addr).await.expect("Failed to bind address");

    tracing::info!("🚀 RustGate listening on http://{}", addr);

    // Start the Axum server
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}