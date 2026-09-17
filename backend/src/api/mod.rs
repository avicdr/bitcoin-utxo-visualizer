pub mod error;
pub mod health;
pub mod node;

use crate::config::AppConfig;
use crate::rpc::client::BitcoinRpcClient;
use axum::{routing::get, Router};
use node::NodeApiState;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub rpc: Arc<BitcoinRpcClient>,
    pub config: AppConfig,
}

pub fn create_router(state: AppState) -> Router {
    let node_state = NodeApiState {
        pool: state.pool.clone(),
        rpc: state.rpc.clone(),
    };

    Router::new()
        .route("/api/health", get(health::health_handler))
        .route(
            "/api/node",
            get(node::get_node_status).with_state(node_state),
        )
        .with_state(state.pool)
}
