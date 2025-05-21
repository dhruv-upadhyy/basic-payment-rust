use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(String),
    #[error("Authentication error: {0}")]
    Auth(String),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Not found: {0}")]
    NotFound(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_code, message) = match self {
            AppError::Database(ref msg) => (
                StatusCode::INTERNAL_SERVER_ERROR, 
                "INTERNAL_SERVER_ERROR", 
                msg.clone()
            ),
            AppError::Auth(ref msg) => (
                StatusCode::UNAUTHORIZED, 
                "AUTH_FAILED", 
                msg.clone()
            ),
            AppError::Validation(ref msg) => (
                StatusCode::BAD_REQUEST, 
                "INVALID_INPUT", 
                msg.clone()
            ),
            AppError::NotFound(ref msg) => (
                StatusCode::NOT_FOUND, 
                "NOT_FOUND", 
                msg.clone()
            ),
        };

        let body = Json(json!({
            "error": {
                "code": error_code,
                "message": message
            }
        }));

        (status, body).into_response()
    }
}

impl From<AppError> for Response {
    fn from(error: AppError) -> Self {
        error.into_response()
    }
} 