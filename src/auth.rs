use axum::{http::{Request, StatusCode}, middleware::Next, response::Response, body::Body};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, TokenData, errors::Error as JwtError};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

fn validate_token(token: &str) -> Result<TokenData<Claims>, JwtError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(b"secret"),
        &Validation::new(Algorithm::HS256),
    )
}

pub async fn jwt_auth(req: Request<Body>, next: Next<>) -> Result<Response, StatusCode> {
    if let Some(auth_header) = req.headers().get("Authorization").and_then(|v| v.to_str().ok()) {
        if let Some(token) = auth_header.strip_prefix("Bearer ") {
            match validate_token(token) {
                Ok(data) => {
                    info!("Valid token for subject: {}", data.claims.sub);
                    return Ok(next.run(req).await);
                }
                Err(err) => {
                    warn!("Token validation failed: {}", err);
                }
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}
