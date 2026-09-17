use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphResponse {
    pub root_id: String,
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    #[serde(rename = "type")]
    pub node_type: String, // "transaction" or "utxo"
    pub position: NodePosition,
    pub data: GraphNodeData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePosition {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GraphNodeData {
    Transaction(TxNodeData),
    Utxo(UtxoNodeData),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxNodeData {
    pub txid: String,
    pub fee: Option<i64>,
    pub fee_rate: Option<f64>,
    pub vsize: i32,
    pub confirmed: bool,
    pub is_coinbase: bool,
    pub block_height: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtxoNodeData {
    pub txid: String,
    pub vout: i32,
    pub outpoint: String,
    pub value_sats: i64,
    pub value_btc: f64,
    pub script_type: String,
    pub is_spent: bool,
    pub address: Option<String>,
    pub spent_by_txid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub id: String,
    pub source: String,
    pub target: String,
    pub label: String,
    #[serde(rename = "type")]
    pub edge_type: String, // "output" or "spend"
    pub animated: bool,
}
