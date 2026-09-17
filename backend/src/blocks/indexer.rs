use crate::rpc::client::BitcoinRpcClient;
use crate::transactions::parser::{parse_rpc_transaction, ParsedTransaction};
use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::PgPool;

pub struct IndexerResult {
    pub height: u64,
    pub block_hash: String,
    pub tx_count: usize,
}

pub async fn index_block_by_height(
    pool: &PgPool,
    rpc: &BitcoinRpcClient,
    height: u64,
) -> Result<IndexerResult> {
    let block_hash = rpc.get_block_hash(height).await?;
    let rpc_block = rpc.get_block_verbose(&block_hash).await?;

    let block_time =
        DateTime::<Utc>::from_timestamp(rpc_block.time as i64, 0).unwrap_or_else(Utc::now);

    // 1. Persist block header
    sqlx::query!(
        r#"
        INSERT INTO blocks (hash, height, prev_hash, timestamp, tx_count, size, weight)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (hash) DO UPDATE SET
            tx_count = EXCLUDED.tx_count,
            size = EXCLUDED.size,
            weight = EXCLUDED.weight
        "#,
        rpc_block.hash,
        rpc_block.height as i64,
        rpc_block.previousblockhash,
        block_time,
        rpc_block.tx.len() as i32,
        rpc_block.size as i32,
        rpc_block.weight as i32,
    )
    .execute(pool)
    .await?;

    // 2. Process each transaction in block
    for txid in &rpc_block.tx {
        let raw_tx = rpc.get_raw_transaction(txid, true).await?;
        let parsed = parse_rpc_transaction(&raw_tx)?;
        persist_transaction(pool, &parsed, Some(&rpc_block.hash), Some(height as i64)).await?;
    }

    // 3. Update sync state
    sqlx::query!(
        r#"
        INSERT INTO sync_state (key, value, updated_at)
        VALUES ('indexed_height', $1, NOW())
        ON CONFLICT (key) DO UPDATE SET
            value = EXCLUDED.value,
            updated_at = NOW()
        "#,
        height.to_string()
    )
    .execute(pool)
    .await?;

    Ok(IndexerResult {
        height,
        block_hash: rpc_block.hash,
        tx_count: rpc_block.tx.len(),
    })
}

pub async fn persist_transaction(
    pool: &PgPool,
    tx: &ParsedTransaction,
    block_hash: Option<&str>,
    block_height: Option<i64>,
) -> Result<()> {
    let mut db_tx = pool.begin().await?;

    // Insert Transaction
    sqlx::query!(
        r#"
        INSERT INTO transactions (
            txid, block_hash, block_height, version, locktime, size, vsize, weight, is_coinbase, status
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 'confirmed')
        ON CONFLICT (txid) DO UPDATE SET
            block_hash = EXCLUDED.block_hash,
            block_height = EXCLUDED.block_height,
            status = 'confirmed'
        "#,
        tx.txid,
        block_hash,
        block_height,
        tx.version,
        tx.locktime as i64,
        tx.size as i32,
        tx.vsize as i32,
        tx.weight as i32,
        tx.is_coinbase,
    )
    .execute(&mut *db_tx)
    .await?;

    // Insert Outputs (UTXO creation)
    for output in &tx.outputs {
        let script_pubkey_bytes = hex::decode(&output.script_pubkey_hex).unwrap_or_default();

        sqlx::query!(
            r#"
            INSERT INTO transaction_outputs (
                txid, vout, value, script_pubkey, script_pubkey_asm, script_type, address, is_spent
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, FALSE)
            ON CONFLICT (txid, vout) DO UPDATE SET
                value = EXCLUDED.value,
                script_pubkey = EXCLUDED.script_pubkey,
                script_pubkey_asm = EXCLUDED.script_pubkey_asm,
                script_type = EXCLUDED.script_type,
                address = EXCLUDED.address
            "#,
            tx.txid,
            output.vout as i32,
            output.value_sats,
            script_pubkey_bytes,
            output.script_pubkey_asm,
            output.script_type,
            output.address,
        )
        .execute(&mut *db_tx)
        .await?;
    }

    // Insert Inputs & Update Spending State
    for input in &tx.inputs {
        let script_sig_bytes = input
            .script_sig_hex
            .as_ref()
            .and_then(|h| hex::decode(h).ok());
        let witness_json = input
            .witness
            .as_ref()
            .and_then(|w| serde_json::to_value(w).ok());

        sqlx::query!(
            r#"
            INSERT INTO transaction_inputs (
                txid, vin, prev_txid, prev_vout, sequence, script_sig, script_sig_asm, witness_items
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (txid, vin) DO UPDATE SET
                prev_txid = EXCLUDED.prev_txid,
                prev_vout = EXCLUDED.prev_vout,
                sequence = EXCLUDED.sequence,
                script_sig = EXCLUDED.script_sig,
                script_sig_asm = EXCLUDED.script_sig_asm,
                witness_items = EXCLUDED.witness_items
            "#,
            tx.txid,
            input.vin as i32,
            input.prev_txid,
            input.prev_vout as i32,
            input.sequence as i64,
            script_sig_bytes,
            input.script_sig_asm,
            witness_json,
        )
        .execute(&mut *db_tx)
        .await?;

        // If this input spends a non-coinbase output, update the spent status on the referenced UTXO
        if !input.is_coinbase {
            sqlx::query!(
                r#"
                UPDATE transaction_outputs
                SET is_spent = TRUE,
                    spent_by_txid = $1,
                    spent_by_vin = $2,
                    spent_at_height = $3
                WHERE txid = $4 AND vout = $5
                "#,
                tx.txid,
                input.vin as i32,
                block_height,
                input.prev_txid,
                input.prev_vout as i32,
            )
            .execute(&mut *db_tx)
            .await?;
        }
    }

    db_tx.commit().await?;
    Ok(())
}

pub async fn sync_to_tip(pool: &PgPool, rpc: &BitcoinRpcClient) -> Result<u64> {
    let blockchain_info = rpc.get_blockchain_info().await?;
    let tip_height = blockchain_info.blocks;

    let current_indexed: Option<i64> = sqlx::query_scalar("SELECT MAX(height) FROM blocks")
        .fetch_optional(pool)
        .await?
        .flatten();

    let start_height = match current_indexed {
        Some(h) => (h + 1) as u64,
        None => 0,
    };

    let mut indexed_count = 0;
    for height in start_height..=tip_height {
        tracing::info!("Indexing block at height #{} / #{}...", height, tip_height);
        index_block_by_height(pool, rpc, height).await?;
        indexed_count += 1;
    }

    Ok(indexed_count)
}
