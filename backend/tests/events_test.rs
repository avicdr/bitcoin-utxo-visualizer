use bitcoin_utxo_backend::events::broadcaster::{EventBroadcaster, SystemEvent};

#[tokio::test]
async fn test_broadcaster_subscribe_and_receive() {
    let broadcaster = EventBroadcaster::new(10);
    let mut rx = broadcaster.subscribe();

    let event = SystemEvent::BlockConnected {
        hash: "0000000000000000000000000000000000000000000000000000000000000101".to_string(),
        height: 101,
        tx_count: 5,
    };

    broadcaster.broadcast(event);

    let received = rx.recv().await.expect("Should receive broadcast event");
    match received {
        SystemEvent::BlockConnected {
            hash,
            height,
            tx_count,
        } => {
            assert_eq!(height, 101);
            assert_eq!(tx_count, 5);
            assert!(hash.ends_with("101"));
        }
        _ => panic!("Expected BlockConnected event"),
    }
}

#[test]
fn test_sse_event_formatting() {
    let event = SystemEvent::TxMempool {
        txid: "4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b".to_string(),
        vsize: 141,
        fee: Some(1500),
    };

    let _sse = EventBroadcaster::to_sse_event(&event).expect("Should format to SSE Event");
    // Verifies conversion doesn't error
    let json = serde_json::to_string(&event).unwrap();
    assert!(json.contains("tx_mempool"));
    assert!(json.contains("4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b"));
}
