use bitcoin_utxo_backend::graph::models::{
    GraphEdge, GraphNode, GraphNodeData, GraphResponse, NodePosition, TxNodeData, UtxoNodeData,
};

#[test]
fn test_graph_serialization_structure() {
    let root_txid = "4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b";
    let utxo_outpoint = format!("{}:0", root_txid);

    let tx_node = GraphNode {
        id: format!("tx:{}", root_txid),
        node_type: "transaction".to_string(),
        position: NodePosition { x: 100.0, y: 200.0 },
        data: GraphNodeData::Transaction(TxNodeData {
            txid: root_txid.to_string(),
            fee: Some(1500),
            fee_rate: Some(10.5),
            vsize: 142,
            confirmed: true,
            is_coinbase: false,
            block_height: Some(101),
        }),
    };

    let utxo_node = GraphNode {
        id: format!("utxo:{}", utxo_outpoint),
        node_type: "utxo".to_string(),
        position: NodePosition { x: 320.0, y: 200.0 },
        data: GraphNodeData::Utxo(UtxoNodeData {
            txid: root_txid.to_string(),
            vout: 0,
            outpoint: utxo_outpoint.clone(),
            value_sats: 50_000_000,
            value_btc: 0.5,
            script_type: "witness_v0_keyhash".to_string(),
            is_spent: false,
            address: Some("bcrt1q...".to_string()),
            spent_by_txid: None,
        }),
    };

    let edge = GraphEdge {
        id: format!("e:tx:{}->utxo:{}", root_txid, utxo_outpoint),
        source: format!("tx:{}", root_txid),
        target: format!("utxo:{}", utxo_outpoint),
        label: "vout:0 (0.50 BTC)".to_string(),
        edge_type: "output".to_string(),
        animated: false,
    };

    let graph = GraphResponse {
        root_id: format!("tx:{}", root_txid),
        nodes: vec![tx_node, utxo_node],
        edges: vec![edge],
        collapsed: false,
        total_count: 2,
    };

    let json = serde_json::to_string(&graph).expect("Should serialize GraphResponse");
    assert!(json.contains("transaction"));
    assert!(json.contains("utxo"));
    assert!(json.contains("vout:0 (0.50 BTC)"));

    let deserialized: GraphResponse =
        serde_json::from_str(&json).expect("Should deserialize GraphResponse");
    assert_eq!(deserialized.nodes.len(), 2);
    assert_eq!(deserialized.edges.len(), 1);
    assert!(!deserialized.collapsed);
}

#[test]
fn test_cluster_node_serialization() {
    use bitcoin_utxo_backend::graph::models::ClusterNodeData;

    let cluster = GraphNode {
        id: "cluster:tx123:inputs".to_string(),
        node_type: "cluster".to_string(),
        position: NodePosition { x: 50.0, y: 50.0 },
        data: GraphNodeData::Cluster(ClusterNodeData {
            label: "+120 Ancestor Inputs".to_string(),
            count: 120,
            parent_id: "tx123".to_string(),
            cluster_type: "inputs".to_string(),
        }),
    };

    let response = GraphResponse {
        root_id: "tx123".to_string(),
        nodes: vec![cluster],
        edges: vec![],
        collapsed: true,
        total_count: 121,
    };

    let json = serde_json::to_string(&response).expect("Should serialize cluster GraphResponse");
    assert!(json.contains("cluster:tx123:inputs"));
    assert!(json.contains("+120 Ancestor Inputs"));
    assert!(json.contains("\"collapsed\":true"));
    assert!(json.contains("\"total_count\":121"));
}
