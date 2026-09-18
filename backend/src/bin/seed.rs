use anyhow::Result;
use chrono::{Duration, Utc};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/bitcoin_utxo".to_string());

    println!("==> Connecting to PostgreSQL at {}", database_url);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("==> Clearing existing test data...");
    sqlx::query!("TRUNCATE TABLE mempool_events, transaction_inputs, transaction_outputs, transactions, blocks CASCADE")
        .execute(&pool)
        .await?;

    let now = Utc::now();

    // -------------------------------------------------------------------------
    // 1. Blocks #100 to #105
    // -------------------------------------------------------------------------
    println!("==> Seeding Blocks #100 to #105...");
    let block_hashes = [
        "00000000000000000001a1b2c3d4e5f600000000000000000000000000000100",
        "00000000000000000002a1b2c3d4e5f600000000000000000000000000000101",
        "00000000000000000003a1b2c3d4e5f600000000000000000000000000000102",
        "00000000000000000004a1b2c3d4e5f600000000000000000000000000000103",
        "00000000000000000005a1b2c3d4e5f600000000000000000000000000000104",
        "00000000000000000006a1b2c3d4e5f600000000000000000000000000000105",
    ];

    for (idx, hash) in block_hashes.iter().enumerate() {
        let height = (100 + idx) as i64;
        let ts = now - Duration::minutes((5 - idx as i64) * 10);
        let prev = if idx == 0 {
            "0000000000000000000000000000000000000000000000000000000000000099"
        } else {
            block_hashes[idx - 1]
        };

        sqlx::query!(
            r#"
            INSERT INTO blocks (hash, height, prev_hash, timestamp, tx_count, size, weight)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            hash,
            height,
            prev,
            ts,
            2,
            1250,
            5000
        )
        .execute(&pool)
        .await?;
    }

    // -------------------------------------------------------------------------
    // 2. Transactions & UTXOs
    // -------------------------------------------------------------------------
    println!("==> Seeding Transactions, Outpoints, and Graph Dependencies...");

    // TX 0: Coinbase at Block 100
    let tx0 = "c010ba5e00000000000000000000000000000000000000000000000000000100";
    sqlx::query!(
        r#"
        INSERT INTO transactions (txid, block_hash, block_height, version, locktime, size, vsize, weight, is_coinbase, fee, fee_rate, status, first_seen)
        VALUES ($1, $2, 100, 2, 0, 160, 160, 640, TRUE, NULL, NULL, 'confirmed', $3)
        "#,
        tx0,
        block_hashes[0],
        now - Duration::minutes(50)
    ).execute(&pool).await?;

    sqlx::query!(
        r#"
        INSERT INTO transaction_inputs (txid, vin, prev_txid, prev_vout, sequence, script_sig, script_sig_asm, witness_items, value)
        VALUES ($1, 0, '0000000000000000000000000000000000000000000000000000000000000000', -1, 4294967295, $2, 'OP_PUSHBYTES_3 03640000', NULL, NULL)
        "#,
        tx0,
        hex::decode("03640000")?
    ).execute(&pool).await?;

    // Coinbase output: 50 BTC (5,000,000,000 sats) -> Spent by TX1
    let tx1 = "1111111111111111111111111111111111111111111111111111111111111111";
    sqlx::query!(
        r#"
        INSERT INTO transaction_outputs (txid, vout, value, script_pubkey, script_pubkey_asm, script_type, address, is_spent, spent_by_txid, spent_by_vin, spent_at_height, spent_at_time)
        VALUES ($1, 0, 5000000000, $2, 'OP_0 1111111111111111111111111111111111111111', 'p2wpkh', 'bcrt1q487777777777777777777777777777770yvh6q', TRUE, $3, 0, 101, $4)
        "#,
        tx0,
        hex::decode("00141111111111111111111111111111111111111111")?,
        tx1,
        now - Duration::minutes(40)
    ).execute(&pool).await?;

    // TX 1: Block 101: Spends Coinbase (50 BTC) -> creates Output 0 (25 BTC) & Output 1 (24.999 BTC), Fee: 100,000 sats
    sqlx::query!(
        r#"
        INSERT INTO transactions (txid, block_hash, block_height, version, locktime, size, vsize, weight, is_coinbase, fee, fee_rate, status, first_seen)
        VALUES ($1, $2, 101, 2, 0, 220, 140, 560, FALSE, 100000, 714.2, 'confirmed', $3)
        "#,
        tx1,
        block_hashes[1],
        now - Duration::minutes(40)
    ).execute(&pool).await?;

    sqlx::query!(
        r#"
        INSERT INTO transaction_inputs (txid, vin, prev_txid, prev_vout, sequence, script_sig, script_sig_asm, witness_items, value)
        VALUES ($1, 0, $2, 0, 4294967295, NULL, NULL, '["3044...", "03..."]', 5000000000)
        "#,
        tx1,
        tx0
    ).execute(&pool).await?;

    let tx2_fanout = "2222222222222222222222222222222222222222222222222222222222222222";
    let tx3_chain = "3333333333333333333333333333333333333333333333333333333333333333";

    // TX1 Output 0 (25 BTC) -> Spent by TX2 (Fan-Out)
    sqlx::query!(
        r#"
        INSERT INTO transaction_outputs (txid, vout, value, script_pubkey, script_pubkey_asm, script_type, address, is_spent, spent_by_txid, spent_by_vin, spent_at_height, spent_at_time)
        VALUES ($1, 0, 2500000000, $2, 'OP_0 2222222222222222222222222222222222222222', 'p2wpkh', 'bcrt1qu8888888888888888888888888888888k2vj6s', TRUE, $3, 0, 102, $4)
        "#,
        tx1,
        hex::decode("00142222222222222222222222222222222222222222")?,
        tx2_fanout,
        now - Duration::minutes(30)
    ).execute(&pool).await?;

    // TX1 Output 1 (24.999 BTC) -> Spent by TX3 (Chain)
    sqlx::query!(
        r#"
        INSERT INTO transaction_outputs (txid, vout, value, script_pubkey, script_pubkey_asm, script_type, address, is_spent, spent_by_txid, spent_by_vin, spent_at_height, spent_at_time)
        VALUES ($1, 1, 2499900000, $2, 'OP_1 3333333333333333333333333333333333333333333333333333333333333333', 'p2tr', 'bcrt1p99999999999999999999999999999999999999999999999999994yfh3g', TRUE, $3, 0, 103, $4)
        "#,
        tx1,
        hex::decode("5120333333333333333333333333333333333333333333333333333333333333")?,
        tx3_chain,
        now - Duration::minutes(20)
    ).execute(&pool).await?;

    // -------------------------------------------------------------------------
    // TX 2: Fan-Out at Block 102 (1 input -> 5 outputs: P2PKH, P2SH, P2WPKH, P2TR, OP_RETURN)
    // -------------------------------------------------------------------------
    println!("==> Seeding Fan-Out Transaction (TX2)...");
    sqlx::query!(
        r#"
        INSERT INTO transactions (txid, block_hash, block_height, version, locktime, size, vsize, weight, is_coinbase, fee, fee_rate, status, first_seen)
        VALUES ($1, $2, 102, 2, 0, 380, 240, 960, FALSE, 50000, 208.3, 'confirmed', $3)
        "#,
        tx2_fanout,
        block_hashes[2],
        now - Duration::minutes(30)
    ).execute(&pool).await?;

    sqlx::query!(
        r#"
        INSERT INTO transaction_inputs (txid, vin, prev_txid, prev_vout, sequence, script_sig, script_sig_asm, witness_items, value)
        VALUES ($1, 0, $2, 0, 4294967295, NULL, NULL, '["3044...", "03..."]', 2500000000)
        "#,
        tx2_fanout,
        tx1
    ).execute(&pool).await?;

    // Out 0: P2PKH (5 BTC) - UNSPENT ACTIVE UTXO
    sqlx::query!(
        r#"
        INSERT INTO transaction_outputs (txid, vout, value, script_pubkey, script_pubkey_asm, script_type, address, is_spent)
        VALUES ($1, 0, 500000000, $2, 'OP_DUP OP_HASH160 4444444444444444444444444444444444444444 OP_EQUALVERIFY OP_CHECKSIG', 'p2pkh', '17VZNX1SN5ntKa8UQFxwQbFeFc3iqRYhem', FALSE)
        "#,
        tx2_fanout,
        hex::decode("76a914444444444444444444444444444444444444444488ac")?
    ).execute(&pool).await?;

    // Out 1: P2SH (5 BTC) - UNSPENT ACTIVE UTXO
    sqlx::query!(
        r#"
        INSERT INTO transaction_outputs (txid, vout, value, script_pubkey, script_pubkey_asm, script_type, address, is_spent)
        VALUES ($1, 1, 500000000, $2, 'OP_HASH160 5555555555555555555555555555555555555555 OP_EQUAL', 'p2sh', '39Jmy5qKzYd7V3pL8E3kQJ6c6z6H1gJ4o1', FALSE)
        "#,
        tx2_fanout,
        hex::decode("a914555555555555555555555555555555555555555587")?
    ).execute(&pool).await?;

    // Out 2: P2WPKH (5 BTC) - UNSPENT ACTIVE UTXO
    sqlx::query!(
        r#"
        INSERT INTO transaction_outputs (txid, vout, value, script_pubkey, script_pubkey_asm, script_type, address, is_spent)
        VALUES ($1, 2, 500000000, $2, 'OP_0 6666666666666666666666666666666666666666', 'p2wpkh', 'bcrt1qeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeqw7k79', FALSE)
        "#,
        tx2_fanout,
        hex::decode("00146666666666666666666666666666666666666666")?
    ).execute(&pool).await?;

    // Out 3: Taproot P2TR (9.9495 BTC) - UNSPENT ACTIVE UTXO
    sqlx::query!(
        r#"
        INSERT INTO transaction_outputs (txid, vout, value, script_pubkey, script_pubkey_asm, script_type, address, is_spent)
        VALUES ($1, 3, 994950000, $2, 'OP_1 7777777777777777777777777777777777777777777777777777777777777777', 'p2tr', 'bcrt1pffffffffffffffffffffffffffffffffffffffffffffffffffffqqwex9z', FALSE)
        "#,
        tx2_fanout,
        hex::decode("5120777777777777777777777777777777777777777777777777777777777777")?
    ).execute(&pool).await?;

    // Out 4: OP_RETURN (0 BTC data carrier) - UNSPENDABLE
    sqlx::query!(
        r#"
        INSERT INTO transaction_outputs (txid, vout, value, script_pubkey, script_pubkey_asm, script_type, address, is_spent)
        VALUES ($1, 4, 0, $2, 'OP_RETURN 426974636f696e205554584f2056697375616c697a6572', 'op_return', NULL, FALSE)
        "#,
        tx2_fanout,
        hex::decode("6a17426974636f696e205554584f2056697375616c697a6572")?
    ).execute(&pool).await?;

    // -------------------------------------------------------------------------
    // TX 3 -> TX 4: Confirmed Transaction Chain
    // -------------------------------------------------------------------------
    println!("==> Seeding Transaction Chain (TX3 -> TX4)...");
    let tx4_chain = "4444444444444444444444444444444444444444444444444444444444444444";

    // TX 3 at Block 103 (Input from TX1:1 = 24.999 BTC)
    sqlx::query!(
        r#"
        INSERT INTO transactions (txid, block_hash, block_height, version, locktime, size, vsize, weight, is_coinbase, fee, fee_rate, status, first_seen)
        VALUES ($1, $2, 103, 2, 0, 210, 135, 540, FALSE, 25000, 185.1, 'confirmed', $3)
        "#,
        tx3_chain,
        block_hashes[3],
        now - Duration::minutes(20)
    ).execute(&pool).await?;

    sqlx::query!(
        r#"
        INSERT INTO transaction_inputs (txid, vin, prev_txid, prev_vout, sequence, script_sig, script_sig_asm, witness_items, value)
        VALUES ($1, 0, $2, 1, 4294967295, NULL, NULL, '["witness_sig..."]', 2499900000)
        "#,
        tx3_chain,
        tx1
    ).execute(&pool).await?;

    // TX 3 Out 0 (24.974 BTC) -> Spent by TX 4
    sqlx::query!(
        r#"
        INSERT INTO transaction_outputs (txid, vout, value, script_pubkey, script_pubkey_asm, script_type, address, is_spent, spent_by_txid, spent_by_vin, spent_at_height, spent_at_time)
        VALUES ($1, 0, 2497400000, $2, 'OP_1 8888888888888888888888888888888888888888888888888888888888888888', 'p2tr', 'bcrt1p0000000000000000000000000000000000000000000000000000v2r6qm', TRUE, $3, 0, 104, $4)
        "#,
        tx3_chain,
        hex::decode("5120888888888888888888888888888888888888888888888888888888888888")?,
        tx4_chain,
        now - Duration::minutes(10)
    ).execute(&pool).await?;

    // TX 4 at Block 104
    sqlx::query!(
        r#"
        INSERT INTO transactions (txid, block_hash, block_height, version, locktime, size, vsize, weight, is_coinbase, fee, fee_rate, status, first_seen)
        VALUES ($1, $2, 104, 2, 0, 210, 135, 540, FALSE, 25000, 185.1, 'confirmed', $3)
        "#,
        tx4_chain,
        block_hashes[4],
        now - Duration::minutes(10)
    ).execute(&pool).await?;

    sqlx::query!(
        r#"
        INSERT INTO transaction_inputs (txid, vin, prev_txid, prev_vout, sequence, script_sig, script_sig_asm, witness_items, value)
        VALUES ($1, 0, $2, 0, 4294967295, NULL, NULL, '["witness_sig..."]', 2497400000)
        "#,
        tx4_chain,
        tx3_chain
    ).execute(&pool).await?;

    // TX 4 Out 0: 20 BTC (UNSPENT ACTIVE UTXO)
    sqlx::query!(
        r#"
        INSERT INTO transaction_outputs (txid, vout, value, script_pubkey, script_pubkey_asm, script_type, address, is_spent)
        VALUES ($1, 0, 2000000000, $2, 'OP_0 9999999999999999999999999999999999999999', 'p2wpkh', 'bcrt1q99999999999999999999999999999999999999', FALSE)
        "#,
        tx4_chain,
        hex::decode("00149999999999999999999999999999999999999999")?
    ).execute(&pool).await?;

    // TX 4 Out 1: 4.949 BTC (Will be spent by Mempool Tx)
    let tx_mem1 = "5555555555555555555555555555555555555555555555555555555555555555";
    sqlx::query!(
        r#"
        INSERT INTO transaction_outputs (txid, vout, value, script_pubkey, script_pubkey_asm, script_type, address, is_spent, spent_by_txid, spent_by_vin)
        VALUES ($1, 1, 494900000, $2, 'OP_1 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa', 'p2tr', 'bcrt1paaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa38j5u8', TRUE, $3, 0)
        "#,
        tx4_chain,
        hex::decode("5120aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")?,
        tx_mem1
    ).execute(&pool).await?;

    // -------------------------------------------------------------------------
    // 3. Mempool 0-Conf Transactions (TX_MEM1 -> TX_MEM2)
    // -------------------------------------------------------------------------
    println!("==> Seeding Mempool 0-conf Transactions (TX_MEM1 -> TX_MEM2)...");
    let tx_mem2 = "6666666666666666666666666666666666666666666666666666666666666666";

    // Mempool Tx 1 (0-conf)
    sqlx::query!(
        r#"
        INSERT INTO transactions (txid, block_hash, block_height, version, locktime, size, vsize, weight, is_coinbase, fee, fee_rate, status, first_seen)
        VALUES ($1, NULL, NULL, 2, 0, 210, 135, 540, FALSE, 15000, 111.1, 'mempool', $2)
        "#,
        tx_mem1,
        now - Duration::minutes(3)
    ).execute(&pool).await?;

    sqlx::query!(
        r#"
        INSERT INTO transaction_inputs (txid, vin, prev_txid, prev_vout, sequence, script_sig, script_sig_asm, witness_items, value)
        VALUES ($1, 0, $2, 1, 4294967294, NULL, NULL, '["mem_sig..."]', 494900000)
        "#,
        tx_mem1,
        tx4_chain
    ).execute(&pool).await?;

    // Mempool Tx 1 Output 0 (4.94885 BTC) -> Spent by Mempool Tx 2
    sqlx::query!(
        r#"
        INSERT INTO transaction_outputs (txid, vout, value, script_pubkey, script_pubkey_asm, script_type, address, is_spent, spent_by_txid, spent_by_vin)
        VALUES ($1, 0, 494885000, $2, 'OP_0 bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb', 'p2wpkh', 'bcrt1qbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb', TRUE, $3, 0)
        "#,
        tx_mem1,
        hex::decode("0014bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb")?,
        tx_mem2
    ).execute(&pool).await?;

    // Mempool Tx 2 (0-conf child)
    sqlx::query!(
        r#"
        INSERT INTO transactions (txid, block_hash, block_height, version, locktime, size, vsize, weight, is_coinbase, fee, fee_rate, status, first_seen)
        VALUES ($1, NULL, NULL, 2, 0, 210, 135, 540, FALSE, 20000, 148.1, 'mempool', $2)
        "#,
        tx_mem2,
        now - Duration::minutes(1)
    ).execute(&pool).await?;

    sqlx::query!(
        r#"
        INSERT INTO transaction_inputs (txid, vin, prev_txid, prev_vout, sequence, script_sig, script_sig_asm, witness_items, value)
        VALUES ($1, 0, $2, 0, 4294967294, NULL, NULL, '["mem_child_sig..."]', 494885000)
        "#,
        tx_mem2,
        tx_mem1
    ).execute(&pool).await?;

    // Mempool Tx 2 Output 0: UNSPENT 0-CONF ACTIVE UTXO
    sqlx::query!(
        r#"
        INSERT INTO transaction_outputs (txid, vout, value, script_pubkey, script_pubkey_asm, script_type, address, is_spent)
        VALUES ($1, 0, 494865000, $2, 'OP_1 cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc', 'p2tr', 'bcrt1pccccccccccccccccccccccccccccccccccccccccccccccccccccqqqq9a4', FALSE)
        "#,
        tx_mem2,
        hex::decode("5120cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc")?
    ).execute(&pool).await?;

    // Log mempool events
    sqlx::query!(
        r#"
        INSERT INTO mempool_events (txid, event_type, timestamp, details)
        VALUES
        ($1, 'tx_added', $3, '{"vsize": 135, "fee": 15000, "fee_rate": 111.1}'),
        ($2, 'tx_added', $4, '{"vsize": 135, "fee": 20000, "fee_rate": 148.1}')
        "#,
        tx_mem1,
        tx_mem2,
        now - Duration::minutes(3),
        now - Duration::minutes(1)
    )
    .execute(&pool)
    .await?;

    // Record sync state
    sqlx::query!(
        r#"
        INSERT INTO sync_state (key, value, updated_at)
        VALUES ('best_block_height', '105', NOW())
        ON CONFLICT (key) DO UPDATE SET value = '105', updated_at = NOW()
        "#
    )
    .execute(&pool)
    .await?;

    println!("==> Seed completed successfully!");
    println!("    Indexed Blocks: 6 (Heights 100 - 105)");
    println!("    Indexed Transactions: 5 confirmed, 2 mempool (0-conf)");
    println!("    Active UTXOs: 6 (P2PKH, P2SH, P2WPKH, P2TR, Mempool)");
    println!("    Educational Graph Root: tx:{}", tx1);

    Ok(())
}
