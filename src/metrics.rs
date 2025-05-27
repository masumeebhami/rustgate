use metrics_exporter_prometheus::PrometheusBuilder;
use std::net::SocketAddr;
use axum::{Router, routing::get};
use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

pub fn init() {
    let builder = PrometheusBuilder::new();
    let handle = builder.install_recorder().unwrap();

    tokio::spawn(async move {
        let addr = SocketAddr::from(([0, 0, 0, 0], 9090));
        let app = Router::new().route(
            "/metrics",
            get(|| async move { handle.render() }),
        );

        let listener = TcpListener::bind(addr).await.unwrap();
        loop {
            let (stream, _) = listener.accept().await.unwrap();
            let app = app.clone();

            tokio::spawn(async move {
                if let Err(err) = http1::Builder::new()
                    .serve_connection(TokioIo::new(stream), app.into_make_service())
                    .await
                {
                    eprintln!("Metrics server error: {:?}", err);
                }
            });
        }
    });
}