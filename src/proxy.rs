use axum::body::Body as AxumBody;
use axum::{
    Router, routing::get, extract::Path, response::IntoResponse,
    http::{Request, StatusCode}, middleware,
};
use hyper::{Uri, Response as HyperResponse, body::Incoming as HyperIncoming};
use hyper_util::client::legacy::{Client, connect::HttpConnector};
use hyper_util::rt::TokioExecutor;
use tower_http::trace::TraceLayer;
use crate::config::Settings;
use crate::auth::jwt_auth;

pub fn create_router(settings: Settings) -> Router {
    Router::new()
        .route("/api/:service/*path", get(move |path| proxy_handler(path, settings.clone())))
        .route_layer(middleware::from_fn(jwt_auth))
        .layer(TraceLayer::new_for_http())
}

async fn proxy_handler(
    Path((service, path)): Path<(String, String)>,
    settings: Settings,
) -> impl IntoResponse {
    if let Some(base_url) = settings.services.get(&service) {
        let target = format!("{}/{}", base_url.trim_end_matches('/'), path);
        if let Ok(uri) = target.parse::<Uri>() {
            let client = Client::builder(TokioExecutor::new()).build(HttpConnector::new());

            let req = Request::builder()
                .method("GET")
                .uri(uri)
                .body(AxumBody::empty())
                .unwrap();

            if let Ok(response) = client.request(req).await {
                let (parts, body): (_, HyperIncoming) = response.into_parts();
                let body = AxumBody::from(body); // ✅ This resolves the Incoming → Body type
                return HyperResponse::from_parts(parts, body);
            }
        }
    }

    HyperResponse::builder()
        .status(StatusCode::BAD_GATEWAY)
        .body(AxumBody::empty())
        .unwrap()
}