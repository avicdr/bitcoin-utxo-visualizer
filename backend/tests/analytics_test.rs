use bitcoin_utxo_backend::analytics::models::{
    ScriptDistributionEntry, UtxoOverviewStats, ValueBucket,
};

#[test]
fn test_utxo_overview_stats_serialization() {
    let stats = UtxoOverviewStats {
        total_indexed_utxos: 500,
        total_indexed_sats: 25_000_000_000,
        total_indexed_btc: 250.0,
        spent_outputs_count: 200,
        unspent_outputs_count: 500,
        spent_ratio: 200.0 / 700.0,
        total_coin_days: 1250.5,
        scope_description: "Indexed local UTXOs.",
    };

    let json = serde_json::to_string(&stats).expect("Should serialize UtxoOverviewStats");
    assert!(json.contains("total_indexed_utxos\":500"));
    assert!(json.contains("total_indexed_btc\":250.0"));
    assert!(json.contains("total_coin_days\":1250.5"));
}

#[test]
fn test_value_bucket_structure() {
    let bucket = ValueBucket {
        range_label: "0.1–1 BTC".to_string(),
        count: 42,
        total_sats: 2_100_000_000,
        total_btc: 21.0,
    };

    let json = serde_json::to_string(&bucket).expect("Should serialize ValueBucket");
    assert!(json.contains("0.1–1 BTC"));
    assert!(json.contains("21.0"));
}

#[test]
fn test_script_distribution_entry() {
    let entry = ScriptDistributionEntry {
        script_type: "p2tr".to_string(),
        count: 150,
        total_sats: 7_500_000_000,
        total_btc: 75.0,
        percentage: 30.0,
    };

    let json = serde_json::to_string(&entry).expect("Should serialize ScriptDistributionEntry");
    assert!(json.contains("p2tr"));
    assert!(json.contains("percentage\":30.0"));
}
