use crate::api::error::ApiError;
use axum::{
    extract::{Path, State},
    Json,
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
pub struct TransactionDetail {
    pub txid: String,
    pub block_hash: Option<String>,
    pub block_height: Option<i64>,
    pub version: i32,
    pub locktime: i64,
    pub size: i32,
    pub vsize: i32,
    pub weight: i32,
    pub is_coinbase: bool,
    pub fee: Option<i64>,
    pub fee_rate: Option<f64>,
    pub status: String,
    pub first_seen: DateTime<Utc>,
    pub inputs: Vec<TxInputDetail>,
    pub outputs: Vec<TxOutputDetail>,
}

#[derive(Serialize)]
pub struct TxInputDetail {
    pub vin: i32,
    pub prev_txid: String,
    pub prev_vout: i32,
    pub sequence: i64,
    pub script_sig_asm: Option<String>,
    pub witness_items: Option<serde_json::Value>,
    pub value: Option<i64>,
}

#[derive(Serialize)]
pub struct TxOutputDetail {
    pub vout: i32,
    pub value: i64,
    pub value_btc: f64,
    pub script_pubkey_asm: String,
    pub script_type: String,
    pub address: Option<String>,
    pub is_spent: bool,
    pub spent_by_txid: Option<String>,
    pub spent_by_vin: Option<i32>,
}

pub async fn get_transaction(
    State(pool): State<PgPool>,
    Path(txid): Path<String>,
) -> Result<Json<TransactionDetail>, ApiError> {
    let tx = sqlx::query!(
        r#"
        SELECT txid, block_hash, block_height, version, locktime, size, vsize, weight,
               is_coinbase, fee, fee_rate, status, first_seen
        FROM transactions
        WHERE txid = $1
        "#,
        txid
    )
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| ApiError::NotFound(format!("Transaction {} not found", txid)))?;

    let inputs = sqlx::query_as!(
        TxInputDetail,
        r#"
        SELECT vin, prev_txid, prev_vout, sequence, script_sig_asm, witness_items, value
        FROM transaction_inputs
        WHERE txid = $1
        ORDER BY vin ASC
        "#,
        txid
    )
    .fetch_all(&pool)
    .await?;

    let outputs_raw = sqlx::query!(
        r#"
        SELECT vout, value, script_pubkey_asm, script_type, address, is_spent,
               spent_by_txid, spent_by_vin
        FROM transaction_outputs
        WHERE txid = $1
        ORDER BY vout ASC
        "#,
        txid
    )
    .fetch_all(&pool)
    .await?;

    let outputs = outputs_raw
        .into_iter()
        .map(|o| TxOutputDetail {
            vout: o.vout,
            value: o.value,
            value_btc: (o.value as f64) / 100_000_000.0,
            script_pubkey_asm: o.script_pubkey_asm,
            script_type: o.script_type,
            address: o.address,
            is_spent: o.is_spent,
            spent_by_txid: o.spent_by_txid,
            spent_by_vin: o.spent_by_vin,
        })
        .collect();

    Ok(Json(TransactionDetail {
        txid: tx.txid,
        block_hash: tx.block_hash,
        block_height: tx.block_height,
        version: tx.version,
        locktime: tx.locktime,
        size: tx.size,
        vsize: tx.vsize,
        weight: tx.weight,
        is_coinbase: tx.is_coinbase,
        fee: tx.fee,
        fee_rate: tx.fee_rate,
        status: tx.status,
        first_seen: tx.first_seen,
        inputs,
        outputs,
    }))
}
