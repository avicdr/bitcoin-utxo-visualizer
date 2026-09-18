use bitcoin_utxo_backend::mempool::tracker::{MempoolEventEntry, MempoolSummary};
use chrono::Utc;

#[test]
fn test_mempool_summary_serialization() {
    let summary = MempoolSummary {
        tx_count: 2,
        txids: vec![
            "00000000000000000000000000000000000000000000000000000000000000aa".to_string(),
            "00000000000000000000000000000000000000000000000000000000000000bb".to_string(),
        ],
        recent_events: vec![MempoolEventEntry {
            id: 1,
            txid: "00000000000000000000000000000000000000000000000000000000000000aa".to_string(),
            event_type: "tx_added".to_string(),
            timestamp: Utc::now(),
        }],
    };

    let json = serde_json::to_string(&summary).expect("Should serialize MempoolSummary");
    assert!(json.contains("tx_count\":2"));
    assert!(json.contains("tx_added"));
}
