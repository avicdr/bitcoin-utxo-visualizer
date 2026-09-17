use crate::api::error::ApiError;
use axum::{
    extract::{Path, State},
    Json,
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
pub struct BlockSummary {
    pub hash: String,
    pub height: i64,
    pub prev_hash: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub tx_count: i32,
    pub size: i32,
    pub weight: i32,
}

pub async fn list_blocks(State(pool): State<PgPool>) -> Result<Json<Vec<BlockSummary>>, ApiError> {
    let rows = sqlx::query_as!(
        BlockSummary,
        r#"
        SELECT hash, height, prev_hash, timestamp, tx_count, size, weight
        FROM blocks
        ORDER BY height DESC
        LIMIT 50
        "#
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(rows))
}

pub async fn get_block_by_height(
    State(pool): State<PgPool>,
    Path(height): Path<i64>,
) -> Result<Json<BlockSummary>, ApiError> {
    let block = sqlx::query_as!(
        BlockSummary,
        r#"
        SELECT hash, height, prev_hash, timestamp, tx_count, size, weight
        FROM blocks
        WHERE height = $1
        "#,
        height
    )
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| ApiError::NotFound(format!("Block at height {} not found", height)))?;

    Ok(Json(block))
}
