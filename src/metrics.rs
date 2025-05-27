use axum::{routing::get, Router};
use metrics_exporter_prometheus::PrometheusBuilder;
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub fn init() {
    let builder = PrometheusBuilder::new();
    let handle = builder.install_recorder().unwrap();

    tokio::spawn(async move {
        let addr = SocketAddr::from(([0, 0, 0, 0], 9090));
        let app = Router::new().route("/metrics", get(|| async move { handle.render() }));

        let listener = TcpListener::bind(addr).await.unwrap();
        tracing::info!("📊 Metrics server listening on http://{}", addr);

        axum::serve(listener, app).await.unwrap();
    });
}