use crate::rpc::client::BitcoinRpcClient;
use crate::transactions::parser::parse_rpc_transaction;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MempoolSummary {
    pub tx_count: usize,
    pub txids: Vec<String>,
    pub recent_events: Vec<MempoolEventEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MempoolEventEntry {
    pub id: i64,
    pub txid: String,
    pub event_type: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub async fn sync_mempool(pool: &PgPool, rpc: &BitcoinRpcClient) -> Result<usize> {
    let mempool_txids = rpc.get_raw_mempool().await?;
    let mut added_count = 0;

    for txid in &mempool_txids {
        let exists = sqlx::query_scalar!("SELECT 1 FROM transactions WHERE txid = $1", txid)
            .fetch_optional(pool)
            .await?
            .is_some();

        if !exists {
            if let Ok(raw_tx) = rpc.get_raw_transaction(txid, true).await {
                if let Ok(parsed) = parse_rpc_transaction(&raw_tx) {
                    let mut db_tx = pool.begin().await?;

                    // Insert transaction as mempool 0-conf
                    sqlx::query!(
                        r#"
                        INSERT INTO transactions (
                            txid, block_hash, block_height, version, locktime, size, vsize, weight, is_coinbase, status
                        )
                        VALUES ($1, NULL, NULL, $2, $3, $4, $5, $6, $7, 'mempool')
                        ON CONFLICT (txid) DO NOTHING
                        "#,
                        parsed.txid,
                        parsed.version,
                        parsed.locktime as i64,
                        parsed.size as i32,
                        parsed.vsize as i32,
                        parsed.weight as i32,
                        parsed.is_coinbase,
                    )
                    .execute(&mut *db_tx)
                    .await?;

                    // Insert unconfirmed outputs
                    for out in &parsed.outputs {
                        let script_pubkey_bytes =
                            hex::decode(&out.script_pubkey_hex).unwrap_or_default();
                        sqlx::query!(
                            r#"
                            INSERT INTO transaction_outputs (
                                txid, vout, value, script_pubkey, script_pubkey_asm, script_type, address, is_spent
                            )
                            VALUES ($1, $2, $3, $4, $5, $6, $7, FALSE)
                            ON CONFLICT (txid, vout) DO NOTHING
                            "#,
                            parsed.txid,
                            out.vout as i32,
                            out.value_sats,
                            script_pubkey_bytes,
                            out.script_pubkey_asm,
                            out.script_type,
                            out.address,
                        )
                        .execute(&mut *db_tx)
                        .await?;
                    }

                    // Insert inputs
                    for inp in &parsed.inputs {
                        let script_sig_bytes = inp
                            .script_sig_hex
                            .as_ref()
                            .and_then(|h| hex::decode(h).ok());
                        let witness_json = inp
                            .witness
                            .as_ref()
                            .and_then(|w| serde_json::to_value(w).ok());

                        sqlx::query!(
                            r#"
                            INSERT INTO transaction_inputs (
                                txid, vin, prev_txid, prev_vout, sequence, script_sig, script_sig_asm, witness_items
                            )
                            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                            ON CONFLICT (txid, vin) DO NOTHING
                            "#,
                            parsed.txid,
                            inp.vin as i32,
                            inp.prev_txid,
                            inp.prev_vout as i32,
                            inp.sequence as i64,
                            script_sig_bytes,
                            inp.script_sig_asm,
                            witness_json,
                        )
                        .execute(&mut *db_tx)
                        .await?;
                    }

                    // Record mempool event
                    sqlx::query!(
                        r#"
                        INSERT INTO mempool_events (txid, event_type)
                        VALUES ($1, 'tx_added')
                        "#,
                        parsed.txid
                    )
                    .execute(&mut *db_tx)
                    .await?;

                    db_tx.commit().await?;
                    added_count += 1;
                }
            }
        }
    }

    Ok(added_count)
}

pub async fn get_mempool_overview(pool: &PgPool) -> Result<MempoolSummary> {
    let rows = sqlx::query_scalar!(
        "SELECT txid FROM transactions WHERE status = 'mempool' ORDER BY first_seen DESC"
    )
    .fetch_all(pool)
    .await?;

    let events = sqlx::query_as!(
        MempoolEventEntry,
        r#"
        SELECT id, txid, event_type, timestamp
        FROM mempool_events
        ORDER BY timestamp DESC
        LIMIT 50
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(MempoolSummary {
        tx_count: rows.len(),
        txids: rows,
        recent_events: events,
    })
}
