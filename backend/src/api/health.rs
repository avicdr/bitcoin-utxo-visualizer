use axum::{extract::State, Json};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub version: &'static str,
    pub database: &'static str,
}

pub async fn health_handler(State(pool): State<PgPool>) -> Json<HealthResponse> {
    let db_status = match sqlx::query("SELECT 1").execute(&pool).await {
        Ok(_) => "connected",
        Err(_) => "disconnected",
    };

    Json(HealthResponse {
        status: "healthy",
        version: env!("CARGO_PKG_VERSION"),
        database: db_status,
    })
}
