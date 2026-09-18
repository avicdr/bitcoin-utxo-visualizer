use crate::analytics::models::{
    AgeBucket, ScriptDistributionEntry, UtxoOverviewStats, ValueBucket,
};
use crate::analytics::service;
use crate::api::error::ApiError;
use axum::{extract::State, Json};
use sqlx::PgPool;

pub async fn get_utxo_overview_handler(
    State(pool): State<PgPool>,
) -> Result<Json<UtxoOverviewStats>, ApiError> {
    let stats = service::get_utxo_overview(&pool).await?;
    Ok(Json(stats))
}

pub async fn get_value_distribution_handler(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<ValueBucket>>, ApiError> {
    let buckets = service::get_value_distribution(&pool).await?;
    Ok(Json(buckets))
}

pub async fn get_age_distribution_handler(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<AgeBucket>>, ApiError> {
    let buckets = service::get_age_distribution(&pool).await?;
    Ok(Json(buckets))
}

pub async fn get_script_distribution_handler(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<ScriptDistributionEntry>>, ApiError> {
    let entries = service::get_script_distribution(&pool).await?;
    Ok(Json(entries))
}
