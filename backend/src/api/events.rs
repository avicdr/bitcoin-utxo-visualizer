use crate::events::broadcaster::EventBroadcaster;
use axum::{
    extract::State,
    response::sse::{Event, KeepAlive, Sse},
};
use futures::stream::Stream;
use std::convert::Infallible;
use std::time::Duration;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt;

pub async fn sse_events_handler(
    State(broadcaster): State<EventBroadcaster>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let receiver = broadcaster.subscribe();
    let stream = BroadcastStream::new(receiver).filter_map(|msg| match msg {
        Ok(event) => match EventBroadcaster::to_sse_event(&event) {
            Ok(sse) => Some(Ok(sse)),
            Err(e) => {
                tracing::warn!("Failed to serialize SSE event: {}", e);
                None
            }
        },
        Err(_) => None,
    });

    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("keep-alive"),
    )
}
