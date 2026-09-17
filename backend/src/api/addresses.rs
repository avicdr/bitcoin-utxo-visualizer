use crate::api::error::ApiError;
use crate::utxos::models::UtxoDetail;
use axum::{
    extract::{Path, State},
    Json,
};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
pub struct AddressAnalysisResponse {
    pub address: String,
    pub total_received_sats: i64,
    pub total_received_btc: f64,
    pub total_spent_sats: i64,
    pub total_spent_btc: f64,
    pub current_balance_sats: i64,
    pub current_balance_btc: f64,
    pub active_utxo_count: usize,
    pub spent_output_count: usize,
    pub active_utxos: Vec<UtxoDetail>,
    pub heuristics_warning: &'static str,
}

pub async fn get_address_analysis(
    State(pool): State<PgPool>,
    Path(address): Path<String>,
) -> Result<Json<AddressAnalysisResponse>, ApiError> {
    // 1. Fetch all outputs (both spent and unspent) created for this address
    let rows = sqlx::query!(
        r#"
        SELECT
            o.txid,
            o.vout,
            o.value,
            o.script_pubkey,
            o.script_pubkey_asm,
            o.script_type,
            o.address,
            o.is_spent,
            o.spent_by_txid,
            o.spent_by_vin,
            o.spent_at_height,
            o.spent_at_time,
            t.block_height AS created_at_height,
            t.first_seen AS created_at_time
        FROM transaction_outputs o
        INNER JOIN transactions t ON o.txid = t.txid
        WHERE o.address = $1
        ORDER BY t.block_height DESC NULLS LAST, o.vout ASC
        "#,
        address
    )
    .fetch_all(&pool)
    .await?;

    let mut total_received_sats: i64 = 0;
    let mut total_spent_sats: i64 = 0;
    let mut active_utxos = Vec::new();
    let mut spent_count = 0;

    for r in rows {
        total_received_sats += r.value;

        if r.is_spent {
            total_spent_sats += r.value;
            spent_count += 1;
        } else {
            let confirmation_state = if r.created_at_height.is_some() {
                "confirmed".to_string()
            } else {
                "unconfirmed".to_string()
            };

            active_utxos.push(UtxoDetail {
                txid: r.txid.clone(),
                vout: r.vout,
                outpoint: format!("{}:{}", r.txid, r.vout),
                value_sats: r.value,
                value_btc: (r.value as f64) / 100_000_000.0,
                script_pubkey_asm: r.script_pubkey_asm,
                script_pubkey_hex: hex::encode(&r.script_pubkey),
                script_type: r.script_type,
                address: r.address,
                is_spent: false,
                confirmation_state,
                created_at_height: r.created_at_height,
                created_at_time: Some(r.created_at_time),
                spent_by: None,
            });
        }
    }

    let current_balance_sats = total_received_sats - total_spent_sats;

    Ok(Json(AddressAnalysisResponse {
        address,
        total_received_sats,
        total_received_btc: (total_received_sats as f64) / 100_000_000.0,
        total_spent_sats,
        total_spent_btc: (total_spent_sats as f64) / 100_000_000.0,
        current_balance_sats,
        current_balance_btc: (current_balance_sats as f64) / 100_000_000.0,
        active_utxo_count: active_utxos.len(),
        spent_output_count: spent_count,
        active_utxos,
        heuristics_warning: "Bitcoin does not maintain account balances. An address is an abstraction over a locking script (scriptPubKey). The balance shown represents the aggregate sum of unspent transaction outputs currently indexed in this database. Address reuse degrades privacy and is an anti-pattern.",
    }))
}
