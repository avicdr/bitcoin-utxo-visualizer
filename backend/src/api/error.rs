use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Bitcoin RPC error: {0}")]
    Rpc(String),

    #[error("Internal server error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.as_str()),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            ApiError::Database(err) => {
                tracing::error!("Database query failed: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database query error")
            }
            ApiError::Rpc(err) => {
                tracing::error!("Bitcoin RPC error: {}", err);
                (StatusCode::BAD_GATEWAY, "Bitcoin node RPC failure")
            }
            ApiError::Internal(err) => {
                tracing::error!("Internal error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        };

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16(),
        }));

        (status, body).into_response()
    }
}
