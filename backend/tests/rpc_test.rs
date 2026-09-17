use bitcoin_utxo_backend::rpc::client::{
    BlockchainInfo, RpcBlockVerbose, RpcRawTransaction, RpcTxOut,
};

#[test]
fn test_deserialize_blockchain_info() {
    let json_data = r#"{
        "chain": "regtest",
        "blocks": 105,
        "headers": 105,
        "bestblockhash": "0f9188f13cb7b2c71f2a335e3a4fc328bf5beb436012afca590b1a11466e2206",
        "difficulty": 4.656542373906925e-10,
        "mediantime": 1726500000,
        "verificationprogress": 1.0,
        "initialblockdownload": false,
        "pruned": false
    }"#;

    let info: BlockchainInfo =
        serde_json::from_str(json_data).expect("Should deserialize BlockchainInfo");
    assert_eq!(info.chain, "regtest");
    assert_eq!(info.blocks, 105);
    assert_eq!(info.headers, 105);
    assert!(!info.initialblockdownload);
}

#[test]
fn test_deserialize_block_verbose() {
    let json_data = r#"{
        "hash": "0f9188f13cb7b2c71f2a335e3a4fc328bf5beb436012afca590b1a11466e2206",
        "confirmations": 1,
        "size": 285,
        "strippedsize": 285,
        "weight": 1140,
        "height": 105,
        "version": 536870912,
        "merkleroot": "4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b",
        "tx": ["4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b"],
        "time": 1726500100,
        "mediantime": 1726500050,
        "nonce": 2,
        "bits": "207fffff",
        "difficulty": 4.656542373906925e-10,
        "chainwork": "00000000000000000000000000000000000000000000000000000000000000d4",
        "n_tx": 1,
        "previousblockhash": "1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b"
    }"#;

    let block: RpcBlockVerbose =
        serde_json::from_str(json_data).expect("Should deserialize RpcBlockVerbose");
    assert_eq!(block.height, 105);
    assert_eq!(block.tx.len(), 1);
    assert_eq!(block.confirmations, 1);
}

#[test]
fn test_deserialize_raw_transaction() {
    let json_data = r#"{
        "txid": "4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b",
        "hash": "4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b",
        "version": 1,
        "size": 134,
        "vsize": 134,
        "weight": 536,
        "locktime": 0,
        "vin": [
            {
                "coinbase": "04ffff001d0104",
                "sequence": 4294967295
            }
        ],
        "vout": [
            {
                "value": 50.00000000,
                "n": 0,
                "script_pub_key": {
                    "asm": "04678afdb0fe5548271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5f OP_CHECKSIG",
                    "hex": "4104678afdb0fe5548271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5fac",
                    "type": "pubkey"
                }
            }
        ],
        "hex": "01000000010000000000000000000000000000000000000000000000000000000000000000ffffffff0704ffff001d0104ffffffff0100f2052a01000000434104678afdb0fe5548271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5fac00000000"
    }"#;

    let tx: RpcRawTransaction =
        serde_json::from_str(json_data).expect("Should deserialize RpcRawTransaction");
    assert_eq!(
        tx.txid,
        "4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b"
    );
    assert_eq!(tx.vin.len(), 1);
    assert_eq!(tx.vout.len(), 1);
    assert_eq!(tx.vout[0].value, 50.0);
}

#[test]
fn test_deserialize_tx_out() {
    let json_data = r#"{
        "bestblock": "0f9188f13cb7b2c71f2a335e3a4fc328bf5beb436012afca590b1a11466e2206",
        "confirmations": 105,
        "value": 50.00000000,
        "script_pub_key": {
            "asm": "OP_DUP OP_HASH160 1111111111111111111111111111111111111111 OP_EQUALVERIFY OP_CHECKSIG",
            "hex": "76a914111111111111111111111111111111111111111188ac",
            "type": "pubkeyhash",
            "address": "bcrt1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqe72cgd"
        },
        "coinbase": true
    }"#;

    let txout: RpcTxOut = serde_json::from_str(json_data).expect("Should deserialize RpcTxOut");
    assert_eq!(txout.confirmations, 105);
    assert_eq!(txout.value, 50.0);
    assert!(txout.coinbase);
}
