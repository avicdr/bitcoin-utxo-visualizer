use crate::rpc::client::BitcoinRpcClient;
use axum::{extract::State, Json};
use serde::Serialize;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct NodeApiState {
    pub pool: PgPool,
    pub rpc: Arc<BitcoinRpcClient>,
}

#[derive(Serialize)]
pub struct NodeStatusResponse {
    pub connected: bool,
    pub network: String,
    pub blocks: u64,
    pub headers: u64,
    pub verification_progress: f64,
    pub subversion: String,
    pub mempool_size: u64,
    pub indexed_height: Option<i64>,
    pub error: Option<String>,
}

pub async fn get_node_status(State(state): State<NodeApiState>) -> Json<NodeStatusResponse> {
    // 1. Query indexed height from database
    let indexed_height: Option<i64> = sqlx::query_scalar("SELECT MAX(height) FROM blocks")
        .fetch_optional(&state.pool)
        .await
        .unwrap_or(None)
        .flatten();

    // 2. Query Bitcoin Core RPC
    match state.rpc.get_blockchain_info().await {
        Ok(info) => {
            let network_info = state.rpc.get_network_info().await.ok();
            let mempool_info = state.rpc.get_mempool_info().await.ok();

            Json(NodeStatusResponse {
                connected: true,
                network: info.chain,
                blocks: info.blocks,
                headers: info.headers,
                verification_progress: info.verificationprogress,
                subversion: network_info
                    .map(|n| n.subversion)
                    .unwrap_or_else(|| "/BitcoinCore/".to_string()),
                mempool_size: mempool_info.map(|m| m.size).unwrap_or(0),
                indexed_height,
                error: None,
            })
        }
        Err(err) => {
            tracing::warn!("Bitcoin Core RPC unreachable: {}", err);
            Json(NodeStatusResponse {
                connected: false,
                network: "unknown".to_string(),
                blocks: 0,
                headers: 0,
                verification_progress: 0.0,
                subversion: "unavailable".to_string(),
                mempool_size: 0,
                indexed_height,
                error: Some(err.to_string()),
            })
        }
    }
}
