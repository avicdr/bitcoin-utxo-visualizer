use crate::utxos::models::{SpendingReference, UtxoDetail};
use anyhow::Result;
use sqlx::PgPool;

pub async fn get_utxo(pool: &PgPool, txid: &str, vout: i32) -> Result<Option<UtxoDetail>> {
    let row = sqlx::query!(
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
            t.first_seen AS created_at_time,
            t.status AS tx_status
        FROM transaction_outputs o
        INNER JOIN transactions t ON o.txid = t.txid
        WHERE o.txid = $1 AND o.vout = $2
        "#,
        txid,
        vout
    )
    .fetch_optional(pool)
    .await?;

    let row = match row {
        Some(r) => r,
        None => return Ok(None),
    };

    let spent_by = if row.is_spent {
        row.spent_by_txid.map(|stxid| SpendingReference {
            spending_txid: stxid,
            spending_vin: row.spent_by_vin.unwrap_or(0),
            spent_at_height: row.spent_at_height,
            spent_at_time: row.spent_at_time,
        })
    } else {
        None
    };

    let confirmation_state = if row.created_at_height.is_some() {
        "confirmed".to_string()
    } else {
        "unconfirmed".to_string()
    };

    Ok(Some(UtxoDetail {
        txid: row.txid.clone(),
        vout: row.vout,
        outpoint: format!("{}:{}", row.txid, row.vout),
        value_sats: row.value,
        value_btc: (row.value as f64) / 100_000_000.0,
        script_pubkey_asm: row.script_pubkey_asm,
        script_pubkey_hex: hex::encode(&row.script_pubkey),
        script_type: row.script_type,
        address: row.address,
        is_spent: row.is_spent,
        confirmation_state,
        created_at_height: row.created_at_height,
        created_at_time: Some(row.created_at_time),
        spent_by,
    }))
}

pub async fn list_active_utxos(pool: &PgPool, limit: i64, offset: i64) -> Result<Vec<UtxoDetail>> {
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
            t.block_height AS created_at_height,
            t.first_seen AS created_at_time
        FROM transaction_outputs o
        INNER JOIN transactions t ON o.txid = t.txid
        WHERE o.is_spent = FALSE
        ORDER BY t.block_height DESC NULLS LAST, o.value DESC
        LIMIT $1 OFFSET $2
        "#,
        limit,
        offset
    )
    .fetch_all(pool)
    .await?;

    let utxos = rows
        .into_iter()
        .map(|r| {
            let confirmation_state = if r.created_at_height.is_some() {
                "confirmed".to_string()
            } else {
                "unconfirmed".to_string()
            };

            UtxoDetail {
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
            }
        })
        .collect();

    Ok(utxos)
}
