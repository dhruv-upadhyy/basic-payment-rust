use axum::{http::{Request, header::AUTHORIZATION}, middleware::Next, body::Body};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use crate::base::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
}

pub async fn auth_middleware(
    mut req: Request<Body>,
    next: Next,
) -> Result<axum::response::Response, AppError> {
    if req.uri().path() == "/users/login" || (req.uri().path() == "/users" && req.method() == &axum::http::Method::POST) {
        return Ok(next.run(req).await);
    }

    let headers = req.headers();

    let auth_header = headers
        .get(AUTHORIZATION)
        .ok_or_else(|| AppError::Auth("Missing authorization header".into()))?
        .to_str()
        .map_err(|_| AppError::Auth("Invalid authorization header".into()))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError::Auth("Invalid authorization header format".into()))?;

    let key = std::env::var("JWT_SECRET")
        .map_err(|_| AppError::Auth("JWT_SECRET not found".into()))?;

    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(key.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| AppError::Auth(format!("Invalid token: {}", e)))?;

    req.extensions_mut().insert(AuthUser {
        user_id: Uuid::parse_str(&claims.claims.sub)
            .map_err(|_| AppError::Auth("Invalid user ID in token".into()))?,
    });

    Ok(next.run(req).await)
}

pub fn create_token(user_id: Uuid) -> Result<String, AppError> {
    let key = std::env::var("JWT_SECRET")
        .map_err(|_| AppError::Auth("JWT_SECRET not found".into()))?;
    let expiration = std::env::var("JWT_EXPIRATION")
        .unwrap_or_else(|_| "24".to_string())
        .parse::<i64>()
        .unwrap_or(24);

    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
        + (expiration * 3600) as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(key.as_bytes()),
    )
    .map_err(|e| AppError::Auth(e.to_string()))
}