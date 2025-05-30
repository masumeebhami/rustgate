use crate::auth::jwt_auth;
use crate::config::Settings;

use axum::{
    body::Body,
    extract::Path,
    http::StatusCode,
    middleware,
    response::Response,
    routing::get,
    Router,
};
use reqwest::Client;
use tower_http::trace::TraceLayer;

pub fn create_router(settings: Settings) -> Router {
    // Wrap settings in an Axum state layer
    Router::new()
        .route("/", get(|| async { "✅ RustGate is running." }))
        .route(
            "/api/{service}/{path..}",
            get(proxy_handler),
        )
        .route_layer(middleware::from_fn(jwt_auth))
        .layer(TraceLayer::new_for_http())
        .with_state(settings)
}

async fn proxy_handler(
    Path((service, path)): Path<(String, String)>,
    axum::extract::State(settings): axum::extract::State<Settings>,
) -> Response<Body> {
    let client = Client::new();

    if let Some(base_url) = settings.services.get(&service) {
        let target = format!("{}/{}", base_url.trim_end_matches('/'), path);

        match client.get(&target).send().await {
            Ok(resp) => {
                let status = resp.status();
                let bytes = resp.bytes().await.unwrap_or_default();

                Response::builder()
                    .status(status)
                    .body(Body::from(bytes))
                    .unwrap()
            }
            Err(_) => Response::builder()
                .status(StatusCode::BAD_GATEWAY)
                .body(Body::empty())
                .unwrap(),
        }
    } else {
        Response::builder()
            .status(StatusCode::BAD_GATEWAY)
            .body(Body::empty())
            .unwrap()
    }
}