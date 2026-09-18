use axum::response::sse::Event;
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum SystemEvent {
    #[serde(rename = "block_connected")]
    BlockConnected {
        hash: String,
        height: u64,
        tx_count: usize,
    },
    #[serde(rename = "tx_mempool")]
    TxMempool {
        txid: String,
        vsize: u64,
        fee: Option<u64>,
    },
    #[serde(rename = "utxo_spent")]
    UtxoSpent {
        outpoint: String,
        spending_txid: String,
        spending_vin: u32,
    },
    #[serde(rename = "heartbeat")]
    Heartbeat {
        indexed_height: Option<i64>,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
}

#[derive(Clone)]
pub struct EventBroadcaster {
    sender: broadcast::Sender<SystemEvent>,
}

impl EventBroadcaster {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    pub fn broadcast(&self, event: SystemEvent) {
        // broadcast sends to all active subscribers; if no subscribers, ignore error
        let _ = self.sender.send(event);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<SystemEvent> {
        self.sender.subscribe()
    }

    pub fn to_sse_event(event: &SystemEvent) -> Result<Event, serde_json::Error> {
        let event_type = match event {
            SystemEvent::BlockConnected { .. } => "block_connected",
            SystemEvent::TxMempool { .. } => "tx_mempool",
            SystemEvent::UtxoSpent { .. } => "utxo_spent",
            SystemEvent::Heartbeat { .. } => "heartbeat",
        };

        let data = serde_json::to_string(event)?;
        Ok(Event::default().event(event_type).data(data))
    }
}
