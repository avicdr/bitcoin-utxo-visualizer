use crate::api::error::ApiError;
use crate::mempool::tracker::{get_mempool_overview, MempoolSummary};
use axum::{extract::State, Json};
use sqlx::PgPool;

pub async fn get_mempool_state(
    State(pool): State<PgPool>,
) -> Result<Json<MempoolSummary>, ApiError> {
    let summary = get_mempool_overview(&pool).await?;
    Ok(Json(summary))
}
