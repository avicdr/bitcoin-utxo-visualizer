pub mod error;
pub mod health;

use axum::{routing::get, Router};
use sqlx::PgPool;

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/api/health", get(health::health_handler))
        .with_state(pool)
}
