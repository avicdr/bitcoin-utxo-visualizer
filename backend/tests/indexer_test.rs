use bitcoin_utxo_backend::rpc::client::{
    RpcRawTransaction, RpcScriptPubKey, RpcTxIn, RpcTxOutEntry,
};
use bitcoin_utxo_backend::transactions::parser::parse_rpc_transaction;

#[test]
fn test_parse_coinbase_transaction() {
    let raw_tx = RpcRawTransaction {
        txid: "e10a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a".to_string(),
        hash: "e10a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a".to_string(),
        version: 2,
        size: 140,
        vsize: 140,
        weight: 560,
        locktime: 0,
        vin: vec![RpcTxIn {
            coinbase: Some("03650000".to_string()),
            txid: None,
            vout: None,
            script_sig: None,
            sequence: 0xffffffff,
            txinwitness: None,
        }],
        vout: vec![RpcTxOutEntry {
            value: 50.0,
            n: 0,
            script_pub_key: RpcScriptPubKey {
                asm: "OP_DUP OP_HASH160 1111111111111111111111111111111111111111 OP_EQUALVERIFY OP_CHECKSIG".to_string(),
                hex: "76a914111111111111111111111111111111111111111188ac".to_string(),
                script_type: "pubkeyhash".to_string(),
                address: Some("bcrt1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqe72cgd".to_string()),
            },
        }],
        hex: "02000000010000...".to_string(),
        blockhash: None,
        confirmations: None,
        time: None,
        blocktime: None,
    };

    let parsed = parse_rpc_transaction(&raw_tx).expect("Failed to parse transaction");
    assert!(parsed.is_coinbase);
    assert_eq!(parsed.inputs.len(), 1);
    assert!(parsed.inputs[0].is_coinbase);
    assert_eq!(parsed.inputs[0].prev_vout, 0xffffffff);
    assert_eq!(parsed.outputs.len(), 1);
    assert_eq!(parsed.outputs[0].value_sats, 5_000_000_000); // 50 BTC in sats
}

#[test]
fn test_parse_multi_input_output_transaction() {
    let raw_tx = RpcRawTransaction {
        txid: "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2".to_string(),
        hash: "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2".to_string(),
        version: 2,
        size: 250,
        vsize: 168,
        weight: 672,
        locktime: 100,
        vin: vec![
            RpcTxIn {
                coinbase: None,
                txid: Some(
                    "0000000000000000000000000000000000000000000000000000000000000001".to_string(),
                ),
                vout: Some(0),
                script_sig: None,
                sequence: 0xfffffffd, // RBF signaled
                txinwitness: Some(vec!["30440220...".to_string(), "02...".to_string()]),
            },
            RpcTxIn {
                coinbase: None,
                txid: Some(
                    "0000000000000000000000000000000000000000000000000000000000000002".to_string(),
                ),
                vout: Some(1),
                script_sig: None,
                sequence: 0xffffffff,
                txinwitness: Some(vec!["30440220...".to_string(), "02...".to_string()]),
            },
        ],
        vout: vec![
            RpcTxOutEntry {
                value: 1.5,
                n: 0,
                script_pub_key: RpcScriptPubKey {
                    asm: "0 20abcdef0123456789abcdef0123456789abcdef01".to_string(),
                    hex: "0014abcdef0123456789abcdef0123456789abcdef01".to_string(),
                    script_type: "witness_v0_keyhash".to_string(),
                    address: Some("bcrt1q...".to_string()),
                },
            },
            RpcTxOutEntry {
                value: 0.4998,
                n: 1,
                script_pub_key: RpcScriptPubKey {
                    asm: "1 20abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789"
                        .to_string(),
                    hex: "5120abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789"
                        .to_string(),
                    script_type: "witness_v1_taproot".to_string(),
                    address: Some("bcrt1p...".to_string()),
                },
            },
        ],
        hex: "0200000002...".to_string(),
        blockhash: Some(
            "00000000000000000000000000000000000000000000000000000000000000aa".to_string(),
        ),
        confirmations: Some(3),
        time: Some(1726500000),
        blocktime: Some(1726500000),
    };

    let parsed = parse_rpc_transaction(&raw_tx).expect("Failed to parse transaction");
    assert!(!parsed.is_coinbase);
    assert_eq!(parsed.inputs.len(), 2);
    assert_eq!(parsed.outputs.len(), 2);

    assert_eq!(parsed.outputs[0].value_sats, 150_000_000); // 1.5 BTC
    assert_eq!(parsed.outputs[1].value_sats, 49_980_000); // 0.4998 BTC
    assert_eq!(parsed.outputs[1].script_type, "witness_v1_taproot");
}
