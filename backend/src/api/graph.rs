use crate::api::error::ApiError;
use crate::graph::builder;
use crate::graph::models::GraphResponse;
use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct GraphQuery {
    pub depth: Option<u32>,
    pub mode: Option<String>,
}

pub async fn get_transaction_graph(
    State(pool): State<PgPool>,
    Path(txid): Path<String>,
    Query(query): Query<GraphQuery>,
) -> Result<Json<GraphResponse>, ApiError> {
    let depth = query.depth.unwrap_or(2).clamp(1, 5);
    let mode = query.mode.unwrap_or_else(|| "dual".to_string());

    let graph = builder::build_utxo_graph(&pool, &txid, depth, &mode)
        .await
        .map_err(|e| ApiError::NotFound(e.to_string()))?;

    Ok(Json(graph))
}
