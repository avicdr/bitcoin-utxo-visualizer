use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtxoOverviewStats {
    pub total_indexed_utxos: i64,
    pub total_indexed_sats: i64,
    pub total_indexed_btc: f64,
    pub spent_outputs_count: i64,
    pub unspent_outputs_count: i64,
    pub spent_ratio: f64,
    pub total_coin_days: f64,
    pub scope_description: &'static str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueBucket {
    pub range_label: String,
    pub count: i64,
    pub total_sats: i64,
    pub total_btc: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgeBucket {
    pub label: String,
    pub block_range: String,
    pub count: i64,
    pub total_sats: i64,
    pub total_btc: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptDistributionEntry {
    pub script_type: String,
    pub count: i64,
    pub total_sats: i64,
    pub total_btc: f64,
    pub percentage: f64,
}
