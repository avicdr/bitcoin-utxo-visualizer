use crate::analytics::models::{
    AgeBucket, ScriptDistributionEntry, UtxoOverviewStats, ValueBucket,
};
use anyhow::Result;
use sqlx::PgPool;

pub async fn get_utxo_overview(pool: &PgPool) -> Result<UtxoOverviewStats> {
    let row = sqlx::query!(
        r#"
        SELECT
            COUNT(*) FILTER (WHERE is_spent = FALSE) AS "unspent_count!",
            COALESCE(SUM(value) FILTER (WHERE is_spent = FALSE), 0)::BIGINT AS "unspent_sats!",
            COUNT(*) FILTER (WHERE is_spent = TRUE) AS "spent_count!"
        FROM transaction_outputs
        "#
    )
    .fetch_one(pool)
    .await?;

    let unspent_count = row.unspent_count;
    let unspent_sats = row.unspent_sats;
    let spent_count = row.spent_count;
    let total_outputs = unspent_count + spent_count;

    let spent_ratio = if total_outputs > 0 {
        (spent_count as f64) / (total_outputs as f64)
    } else {
        0.0
    };

    let tip_height: i64 = sqlx::query_scalar!("SELECT COALESCE(MAX(height), 0) FROM blocks")
        .fetch_one(pool)
        .await?
        .unwrap_or(0);

    let coin_days_row = sqlx::query!(
        r#"
        SELECT
            COALESCE(SUM((o.value / 100000000.0) * GREATEST(0, ($1 - COALESCE(t.block_height, $1)) / 144.0)), 0.0)::DOUBLE PRECISION AS "total_coin_days!"
        FROM transaction_outputs o
        INNER JOIN transactions t ON o.txid = t.txid
        WHERE o.is_spent = FALSE
        "#,
        tip_height
    )
    .fetch_one(pool)
    .await?;

    let total_coin_days = coin_days_row.total_coin_days;

    Ok(UtxoOverviewStats {
        total_indexed_utxos: unspent_count,
        total_indexed_sats: unspent_sats,
        total_indexed_btc: (unspent_sats as f64) / 100_000_000.0,
        spent_outputs_count: spent_count,
        unspent_outputs_count: unspent_count,
        spent_ratio,
        total_coin_days,
        scope_description:
            "Statistics calculated strictly across outputs indexed in the local database.",
    })
}

pub async fn get_value_distribution(pool: &PgPool) -> Result<Vec<ValueBucket>> {
    let rows = sqlx::query!(
        r#"
        SELECT
            CASE
                WHEN value < 100000 THEN '< 0.001 BTC'
                WHEN value >= 100000 AND value < 1000000 THEN '0.001–0.01 BTC'
                WHEN value >= 1000000 AND value < 10000000 THEN '0.01–0.1 BTC'
                WHEN value >= 10000000 AND value < 100000000 THEN '0.1–1 BTC'
                WHEN value >= 100000000 AND value < 1000000000 THEN '1–10 BTC'
                ELSE '10+ BTC'
            END AS "bucket!",
            COUNT(*) AS "count!",
            COALESCE(SUM(value), 0)::BIGINT AS "total_sats!"
        FROM transaction_outputs
        WHERE is_spent = FALSE
        GROUP BY 1
        ORDER BY MIN(value) ASC
        "#
    )
    .fetch_all(pool)
    .await?;

    let buckets = rows
        .into_iter()
        .map(|r| ValueBucket {
            range_label: r.bucket,
            count: r.count,
            total_sats: r.total_sats,
            total_btc: (r.total_sats as f64) / 100_000_000.0,
        })
        .collect();

    Ok(buckets)
}

pub async fn get_age_distribution(pool: &PgPool) -> Result<Vec<AgeBucket>> {
    let tip_height: i64 = sqlx::query_scalar!("SELECT COALESCE(MAX(height), 0) FROM blocks")
        .fetch_one(pool)
        .await?
        .unwrap_or(0);

    let rows = sqlx::query!(
        r#"
        SELECT
            CASE
                WHEN ($1 - COALESCE(t.block_height, $1)) < 144 THEN '0–1 day'
                WHEN ($1 - COALESCE(t.block_height, $1)) < 1008 THEN '1–7 days'
                WHEN ($1 - COALESCE(t.block_height, $1)) < 4320 THEN '7–30 days'
                WHEN ($1 - COALESCE(t.block_height, $1)) < 12960 THEN '30–90 days'
                WHEN ($1 - COALESCE(t.block_height, $1)) < 52560 THEN '90–365 days'
                ELSE '365+ days'
            END AS "bucket!",
            CASE
                WHEN ($1 - COALESCE(t.block_height, $1)) < 144 THEN '< 144 blocks'
                WHEN ($1 - COALESCE(t.block_height, $1)) < 1008 THEN '144–1,008 blocks'
                WHEN ($1 - COALESCE(t.block_height, $1)) < 4320 THEN '1,008–4,320 blocks'
                WHEN ($1 - COALESCE(t.block_height, $1)) < 12960 THEN '4,320–12,960 blocks'
                WHEN ($1 - COALESCE(t.block_height, $1)) < 52560 THEN '12,960–52,560 blocks'
                ELSE '52,560+ blocks'
            END AS "block_range!",
            COUNT(*) AS "count!",
            COALESCE(SUM(o.value), 0)::BIGINT AS "total_sats!"
        FROM transaction_outputs o
        INNER JOIN transactions t ON o.txid = t.txid
        WHERE o.is_spent = FALSE
        GROUP BY 1, 2
        "#,
        tip_height
    )
    .fetch_all(pool)
    .await?;

    let buckets = rows
        .into_iter()
        .map(|r| AgeBucket {
            label: r.bucket,
            block_range: r.block_range,
            count: r.count,
            total_sats: r.total_sats,
            total_btc: (r.total_sats as f64) / 100_000_000.0,
        })
        .collect();

    Ok(buckets)
}

pub async fn get_script_distribution(pool: &PgPool) -> Result<Vec<ScriptDistributionEntry>> {
    let total_unspent: i64 =
        sqlx::query_scalar!("SELECT COUNT(*) FROM transaction_outputs WHERE is_spent = FALSE")
            .fetch_one(pool)
            .await?
            .unwrap_or(0);

    let rows = sqlx::query!(
        r#"
        SELECT
            script_type AS "script_type!",
            COUNT(*) AS "count!",
            COALESCE(SUM(value), 0)::BIGINT AS "total_sats!"
        FROM transaction_outputs
        WHERE is_spent = FALSE
        GROUP BY script_type
        ORDER BY COUNT(*) DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    let entries = rows
        .into_iter()
        .map(|r| {
            let percentage = if total_unspent > 0 {
                ((r.count as f64) / (total_unspent as f64)) * 100.0
            } else {
                0.0
            };

            ScriptDistributionEntry {
                script_type: r.script_type,
                count: r.count,
                total_sats: r.total_sats,
                total_btc: (r.total_sats as f64) / 100_000_000.0,
                percentage,
            }
        })
        .collect();

    Ok(entries)
}
