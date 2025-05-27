// src/main.rs
use std::net::SocketAddr;
use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use axum::Router;
use tower::ServiceExt;
use hyper::{Request};
use axum::body::Body;

mod auth;
mod config;
mod proxy;
mod router;
mod metrics;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    metrics::init();
    let settings = config::load();

    let app = router::create_router(settings).layer(TraceLayer::new_for_http());

    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    tracing::info!("🚀 RustGate listening on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let app = app.clone();

        tokio::spawn(async move {
            let stream = TokioIo::new(stream);
            let service = tower::service_fn(|req: Request<hyper::body::Incoming>| {
                let mut app = app.clone();
                async move {
                    let axum_req = req.map(|incoming| axum::body::Body::wrap_stream(hyper::body::Body::into_stream(incoming)));
                    app.call(axum_req).await
                }
            });

            if let Err(err) = http1::Builder::new()
                .serve_connection(stream, service)
                .await
            {
                eprintln!("Server error: {:?}", err);
            }
        });
    }
}
