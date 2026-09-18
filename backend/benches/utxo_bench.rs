use bitcoin_utxo_backend::graph::models::{
    GraphEdge, GraphNode, GraphNodeData, GraphResponse, NodePosition, TxNodeData, UtxoNodeData,
};
use bitcoin_utxo_backend::rpc::client::{
    RpcRawTransaction, RpcScriptPubKey, RpcTxIn, RpcTxOutEntry,
};
use bitcoin_utxo_backend::scripts::analyzer::analyze_script_bytes;
use bitcoin_utxo_backend::transactions::parser::parse_rpc_transaction;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_script_analysis(c: &mut Criterion) {
    let p2pkh_bytes = hex::decode("76a914111111111111111111111111111111111111111188ac").unwrap();
    let p2wpkh_bytes = hex::decode("00143333333333333333333333333333333333333333").unwrap();
    let p2tr_bytes =
        hex::decode("51205555555555555555555555555555555555555555555555555555555555555555")
            .unwrap();
    let op_return_bytes = hex::decode("6a10626974636f696e5f76697375616c697a6572").unwrap();

    let mut group = c.benchmark_group("script_analysis");

    group.bench_function("p2pkh", |b| {
        b.iter(|| analyze_script_bytes(black_box(&p2pkh_bytes)))
    });

    group.bench_function("p2wpkh", |b| {
        b.iter(|| analyze_script_bytes(black_box(&p2wpkh_bytes)))
    });

    group.bench_function("p2tr", |b| {
        b.iter(|| analyze_script_bytes(black_box(&p2tr_bytes)))
    });

    group.bench_function("op_return", |b| {
        b.iter(|| analyze_script_bytes(black_box(&op_return_bytes)))
    });

    group.finish();
}

fn bench_transaction_parsing(c: &mut Criterion) {
    let tx = RpcRawTransaction {
        txid: "4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b".to_string(),
        hash: "4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b".to_string(),
        version: 2,
        size: 250,
        vsize: 165,
        weight: 660,
        locktime: 0,
        vin: vec![RpcTxIn {
            txid: Some(
                "1111111111111111111111111111111111111111111111111111111111111111".to_string(),
            ),
            vout: Some(0),
            coinbase: None,
            script_sig: None,
            sequence: 0xfffffffe,
            txinwitness: Some(vec!["304402...".to_string(), "03...".to_string()]),
        }],
        vout: vec![
            RpcTxOutEntry {
                value: 0.5,
                n: 0,
                script_pub_key: RpcScriptPubKey {
                    asm: "OP_0 1111...".to_string(),
                    hex: "00141111111111111111111111111111111111111111".to_string(),
                    script_type: "witness_v0_keyhash".to_string(),
                    address: Some("bcrt1q...".to_string()),
                },
            },
            RpcTxOutEntry {
                value: 0.499,
                n: 1,
                script_pub_key: RpcScriptPubKey {
                    asm: "OP_0 2222...".to_string(),
                    hex: "00142222222222222222222222222222222222222222".to_string(),
                    script_type: "witness_v0_keyhash".to_string(),
                    address: Some("bcrt1q_change...".to_string()),
                },
            },
        ],
        hex: "0200000001...".to_string(),
        blockhash: Some(
            "0000000000000000000000000000000000000000000000000000000000000001".to_string(),
        ),
        confirmations: Some(10),
        time: Some(1600000000),
        blocktime: Some(1600000000),
    };

    c.bench_function("transaction_parsing", |b| {
        b.iter(|| parse_rpc_transaction(black_box(&tx)))
    });
}

fn bench_graph_serialization(c: &mut Criterion) {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    for i in 0..100 {
        let txid = format!("{:064x}", i);
        nodes.push(GraphNode {
            id: format!("tx:{}", txid),
            node_type: "transaction".to_string(),
            position: NodePosition {
                x: (i as f64) * 50.0,
                y: 100.0,
            },
            data: GraphNodeData::Transaction(TxNodeData {
                txid: txid.clone(),
                fee: Some(1000),
                fee_rate: Some(5.0),
                vsize: 200,
                confirmed: true,
                is_coinbase: false,
                block_height: Some(100),
            }),
        });

        nodes.push(GraphNode {
            id: format!("utxo:{}:0", txid),
            node_type: "utxo".to_string(),
            position: NodePosition {
                x: (i as f64) * 50.0,
                y: 200.0,
            },
            data: GraphNodeData::Utxo(UtxoNodeData {
                txid: txid.clone(),
                vout: 0,
                outpoint: format!("{}:0", txid),
                value_sats: 50_000_000,
                value_btc: 0.5,
                script_type: "witness_v0_keyhash".to_string(),
                is_spent: false,
                address: Some("bcrt1q...".to_string()),
                spent_by_txid: None,
            }),
        });

        edges.push(GraphEdge {
            id: format!("e:tx:{}->utxo:{}:0", txid, txid),
            source: format!("tx:{}", txid),
            target: format!("utxo:{}:0", txid),
            label: "vout:0".to_string(),
            edge_type: "output".to_string(),
            animated: false,
        });
    }

    let graph = GraphResponse {
        root_id: "tx:0".to_string(),
        nodes,
        edges,
        collapsed: false,
        total_count: 200,
    };

    c.bench_function("graph_json_serialization", |b| {
        b.iter(|| serde_json::to_string(black_box(&graph)).unwrap())
    });
}

criterion_group!(
    benches,
    bench_script_analysis,
    bench_transaction_parsing,
    bench_graph_serialization
);
criterion_main!(benches);
