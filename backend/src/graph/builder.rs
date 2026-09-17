use crate::graph::models::{
    GraphEdge, GraphNode, GraphNodeData, GraphResponse, NodePosition, TxNodeData, UtxoNodeData,
};
use anyhow::{anyhow, Result};
use sqlx::PgPool;
use std::collections::{HashSet, VecDeque};

pub async fn build_utxo_graph(
    pool: &PgPool,
    root_txid: &str,
    max_depth: u32,
    _mode: &str,
) -> Result<GraphResponse> {
    let mut nodes: Vec<GraphNode> = Vec::new();
    let mut edges: Vec<GraphEdge> = Vec::new();
    let mut seen_nodes: HashSet<String> = HashSet::new();
    let mut seen_edges: HashSet<String> = HashSet::new();

    // 1. Verify root transaction exists
    let root_tx = sqlx::query!(
        r#"
        SELECT txid, block_height, vsize, is_coinbase, fee, fee_rate, status
        FROM transactions
        WHERE txid = $1
        "#,
        root_txid
    )
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| anyhow!("Root transaction {} not found", root_txid))?;

    let root_tx_node_id = format!("tx:{}", root_tx.txid);
    seen_nodes.insert(root_tx_node_id.clone());
    nodes.push(GraphNode {
        id: root_tx_node_id.clone(),
        node_type: "transaction".to_string(),
        position: NodePosition { x: 400.0, y: 300.0 },
        data: GraphNodeData::Transaction(TxNodeData {
            txid: root_tx.txid.clone(),
            fee: root_tx.fee,
            fee_rate: root_tx.fee_rate,
            vsize: root_tx.vsize,
            confirmed: root_tx.status == "confirmed",
            is_coinbase: root_tx.is_coinbase,
            block_height: root_tx.block_height,
        }),
    });

    // 2. BFS Traversal for Ancestors (Inputs)
    let mut queue: VecDeque<(String, u32, f64, f64)> = VecDeque::new();
    queue.push_back((root_tx.txid.clone(), 0, 400.0, 300.0));

    while let Some((curr_txid, depth, tx_x, tx_y)) = queue.pop_front() {
        if depth >= max_depth {
            continue;
        }

        // Fetch inputs for curr_txid
        let inputs = sqlx::query!(
            r#"
            SELECT vin, prev_txid, prev_vout, value
            FROM transaction_inputs
            WHERE txid = $1
            ORDER BY vin ASC
            "#,
            curr_txid
        )
        .fetch_all(pool)
        .await?;

        let input_count = inputs.len();
        for (i, inp) in inputs.into_iter().enumerate() {
            // Skip null coinbase outpoint
            if inp.prev_txid == "0000000000000000000000000000000000000000000000000000000000000000" {
                continue;
            }

            let utxo_node_id = format!("utxo:{}:{}", inp.prev_txid, inp.prev_vout);
            let edge_id = format!("e:{}->tx:{}", utxo_node_id, curr_txid);

            // Compute positions
            let offset_y = ((i as f64) - ((input_count as f64) - 1.0) / 2.0) * 120.0;
            let utxo_x = tx_x - 220.0;
            let utxo_y = tx_y + offset_y;

            if !seen_nodes.contains(&utxo_node_id) {
                seen_nodes.insert(utxo_node_id.clone());

                // Fetch details of referenced output
                let prev_out = sqlx::query!(
                    r#"
                    SELECT value, script_type, address, is_spent, spent_by_txid
                    FROM transaction_outputs
                    WHERE txid = $1 AND vout = $2
                    "#,
                    inp.prev_txid,
                    inp.prev_vout
                )
                .fetch_optional(pool)
                .await?;

                if let Some(out) = prev_out {
                    nodes.push(GraphNode {
                        id: utxo_node_id.clone(),
                        node_type: "utxo".to_string(),
                        position: NodePosition {
                            x: utxo_x,
                            y: utxo_y,
                        },
                        data: GraphNodeData::Utxo(UtxoNodeData {
                            txid: inp.prev_txid.clone(),
                            vout: inp.prev_vout,
                            outpoint: format!("{}:{}", inp.prev_txid, inp.prev_vout),
                            value_sats: out.value,
                            value_btc: (out.value as f64) / 100_000_000.0,
                            script_type: out.script_type,
                            is_spent: out.is_spent,
                            address: out.address,
                            spent_by_txid: out.spent_by_txid,
                        }),
                    });
                }
            }

            if !seen_edges.contains(&edge_id) {
                seen_edges.insert(edge_id.clone());
                edges.push(GraphEdge {
                    id: edge_id,
                    source: utxo_node_id.clone(),
                    target: format!("tx:{}", curr_txid),
                    label: format!("vin:{}", inp.vin),
                    edge_type: "spend".to_string(),
                    animated: false,
                });
            }

            // Fetch parent transaction
            let parent_tx_id = format!("tx:{}", inp.prev_txid);
            let parent_tx_edge_id = format!("e:{}->{}", parent_tx_id, utxo_node_id);
            let parent_x = utxo_x - 220.0;
            let parent_y = utxo_y;

            if !seen_nodes.contains(&parent_tx_id) {
                seen_nodes.insert(parent_tx_id.clone());
                if let Some(ptx) = sqlx::query!(
                    r#"
                    SELECT txid, block_height, vsize, is_coinbase, fee, fee_rate, status
                    FROM transactions
                    WHERE txid = $1
                    "#,
                    inp.prev_txid
                )
                .fetch_optional(pool)
                .await?
                {
                    nodes.push(GraphNode {
                        id: parent_tx_id.clone(),
                        node_type: "transaction".to_string(),
                        position: NodePosition {
                            x: parent_x,
                            y: parent_y,
                        },
                        data: GraphNodeData::Transaction(TxNodeData {
                            txid: ptx.txid.clone(),
                            fee: ptx.fee,
                            fee_rate: ptx.fee_rate,
                            vsize: ptx.vsize,
                            confirmed: ptx.status == "confirmed",
                            is_coinbase: ptx.is_coinbase,
                            block_height: ptx.block_height,
                        }),
                    });

                    // Add to traversal queue
                    queue.push_back((inp.prev_txid.clone(), depth + 1, parent_x, parent_y));
                }
            }

            if !seen_edges.contains(&parent_tx_edge_id) {
                seen_edges.insert(parent_tx_edge_id.clone());
                edges.push(GraphEdge {
                    id: parent_tx_edge_id,
                    source: parent_tx_id,
                    target: utxo_node_id,
                    label: format!("vout:{}", inp.prev_vout),
                    edge_type: "output".to_string(),
                    animated: false,
                });
            }
        }
    }

    // 3. Forward Traversal for Outputs & Descendants
    let outputs = sqlx::query!(
        r#"
        SELECT vout, value, script_type, address, is_spent, spent_by_txid, spent_by_vin
        FROM transaction_outputs
        WHERE txid = $1
        ORDER BY vout ASC
        "#,
        root_txid
    )
    .fetch_all(pool)
    .await?;

    let output_count = outputs.len();
    for (j, out) in outputs.into_iter().enumerate() {
        let utxo_node_id = format!("utxo:{}:{}", root_txid, out.vout);
        let edge_id = format!("e:tx:{}->{}", root_txid, utxo_node_id);

        let offset_y = ((j as f64) - ((output_count as f64) - 1.0) / 2.0) * 120.0;
        let utxo_x = 400.0 + 220.0;
        let utxo_y = 300.0 + offset_y;

        if !seen_nodes.contains(&utxo_node_id) {
            seen_nodes.insert(utxo_node_id.clone());
            nodes.push(GraphNode {
                id: utxo_node_id.clone(),
                node_type: "utxo".to_string(),
                position: NodePosition {
                    x: utxo_x,
                    y: utxo_y,
                },
                data: GraphNodeData::Utxo(UtxoNodeData {
                    txid: root_txid.to_string(),
                    vout: out.vout,
                    outpoint: format!("{}:{}", root_txid, out.vout),
                    value_sats: out.value,
                    value_btc: (out.value as f64) / 100_000_000.0,
                    script_type: out.script_type,
                    is_spent: out.is_spent,
                    address: out.address,
                    spent_by_txid: out.spent_by_txid.clone(),
                }),
            });
        }

        if !seen_edges.contains(&edge_id) {
            seen_edges.insert(edge_id.clone());
            edges.push(GraphEdge {
                id: edge_id,
                source: format!("tx:{}", root_txid),
                target: utxo_node_id.clone(),
                label: format!("vout:{}", out.vout),
                edge_type: "output".to_string(),
                animated: false,
            });
        }

        // If spent, resolve spending child transaction
        if let Some(child_txid) = out.spent_by_txid {
            let child_node_id = format!("tx:{}", child_txid);
            let child_edge_id = format!("e:{}->{}", utxo_node_id, child_node_id);
            let child_x = utxo_x + 220.0;
            let child_y = utxo_y;

            if !seen_nodes.contains(&child_node_id) {
                seen_nodes.insert(child_node_id.clone());
                if let Some(ctx) = sqlx::query!(
                    r#"
                    SELECT txid, block_height, vsize, is_coinbase, fee, fee_rate, status
                    FROM transactions
                    WHERE txid = $1
                    "#,
                    child_txid
                )
                .fetch_optional(pool)
                .await?
                {
                    nodes.push(GraphNode {
                        id: child_node_id.clone(),
                        node_type: "transaction".to_string(),
                        position: NodePosition {
                            x: child_x,
                            y: child_y,
                        },
                        data: GraphNodeData::Transaction(TxNodeData {
                            txid: ctx.txid.clone(),
                            fee: ctx.fee,
                            fee_rate: ctx.fee_rate,
                            vsize: ctx.vsize,
                            confirmed: ctx.status == "confirmed",
                            is_coinbase: ctx.is_coinbase,
                            block_height: ctx.block_height,
                        }),
                    });
                }
            }

            if !seen_edges.contains(&child_edge_id) {
                seen_edges.insert(child_edge_id.clone());
                edges.push(GraphEdge {
                    id: child_edge_id,
                    source: utxo_node_id,
                    target: child_node_id,
                    label: format!("vin:{}", out.spent_by_vin.unwrap_or(0)),
                    edge_type: "spend".to_string(),
                    animated: false,
                });
            }
        }
    }

    Ok(GraphResponse {
        root_id: format!("tx:{}", root_txid),
        nodes,
        edges,
    })
}
