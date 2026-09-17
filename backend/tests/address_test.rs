use bitcoin_utxo_backend::api::addresses::AddressAnalysisResponse;

#[test]
fn test_address_analysis_serialization() {
    let resp = AddressAnalysisResponse {
        address: "bcrt1q4a5e1e4baab89f3a32518a88c31bc87f618f7".to_string(),
        total_received_sats: 150_000_000,
        total_received_btc: 1.5,
        total_spent_sats: 50_000_000,
        total_spent_btc: 0.5,
        current_balance_sats: 100_000_000,
        current_balance_btc: 1.0,
        active_utxo_count: 1,
        spent_output_count: 1,
        active_utxos: vec![],
        heuristics_warning: "Bitcoin does not maintain account balances.",
    };

    let json = serde_json::to_string(&resp).expect("Should serialize AddressAnalysisResponse");
    assert!(json.contains("bcrt1q4a5e1e4baab89f3a32518a88c31bc87f618f7"));
    assert!(json.contains("heuristics_warning"));
    assert!(json.contains("current_balance_sats"));
}
