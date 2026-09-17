pub mod blocks;
pub mod error;
pub mod graph;
pub mod health;
pub mod node;
pub mod scripts;
pub mod transactions;
pub mod utxos;

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
        .route("/api/blocks", get(blocks::list_blocks))
        .route("/api/blocks/:height", get(blocks::get_block_by_height))
        .route(
            "/api/transactions/:txid",
            get(transactions::get_transaction),
        )
        .route(
            "/api/transactions/:txid/graph",
            get(graph::get_transaction_graph),
        )
        .route("/api/utxos", get(utxos::list_utxos))
        .route("/api/utxos/:txid/:vout", get(utxos::get_utxo))
        .route("/api/utxos/:txid/:vout/spend", get(utxos::get_utxo_spend))
        .route("/api/scripts/decode", get(scripts::decode_script))
        .with_state(state.pool)
}
