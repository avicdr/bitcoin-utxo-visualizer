use crate::api::error::ApiError;
use crate::utxos::models::{SpendingReference, UtxoDetail};
use crate::utxos::service;
use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct UtxoPagination {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub async fn list_utxos(
    State(pool): State<PgPool>,
    Query(params): Query<UtxoPagination>,
) -> Result<Json<Vec<UtxoDetail>>, ApiError> {
    let limit = params.limit.unwrap_or(50).clamp(1, 100);
    let offset = params.offset.unwrap_or(0).max(0);

    let utxos = service::list_active_utxos(&pool, limit, offset).await?;
    Ok(Json(utxos))
}

pub async fn get_utxo(
    State(pool): State<PgPool>,
    Path((txid, vout)): Path<(String, i32)>,
) -> Result<Json<UtxoDetail>, ApiError> {
    let utxo = service::get_utxo(&pool, &txid, vout)
        .await?
        .ok_or_else(|| ApiError::NotFound(format!("UTXO outpoint {}:{} not found", txid, vout)))?;

    Ok(Json(utxo))
}

pub async fn get_utxo_spend(
    State(pool): State<PgPool>,
    Path((txid, vout)): Path<(String, i32)>,
) -> Result<Json<Option<SpendingReference>>, ApiError> {
    let utxo = service::get_utxo(&pool, &txid, vout)
        .await?
        .ok_or_else(|| ApiError::NotFound(format!("UTXO outpoint {}:{} not found", txid, vout)))?;

    Ok(Json(utxo.spent_by))
}
