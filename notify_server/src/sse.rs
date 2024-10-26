use std::{convert::Infallible, time::Duration};

use axum::{
    extract::State,
    response::{sse::Event, Sse},
    Extension,
};
use axum_extra::{headers, TypedHeader};
use core_lib::User;
use futures::stream::Stream;
use tokio::sync::broadcast;
use tokio_stream::{wrappers::BroadcastStream, StreamExt};
use tracing::info;

use crate::{AppEvent, AppState};

pub(crate) async fn sse_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    info!("`{}` connected", user_agent.as_str());

    // A `Stream` that repeats an event every second
    //
    // You can also create streams from tokio channels using the wrappers in
    // https://docs.rs/tokio-stream
    let user_id = user.id as u64;
    let users = &state.users;
    let rx = if let Some(tx) = users.get(&user_id) {
        tx.subscribe()
    } else {
        let (tx, rx) = broadcast::channel(256);
        state.users.insert(user_id, tx);
        rx
    };

    let stream = BroadcastStream::new(rx).filter_map(|v| v.ok()).map(|v| {
        let name = match v.as_ref() {
            AppEvent::NewChat(_) => "NewChat",
            AppEvent::NewMessage(_) => "NewMessage",
            AppEvent::AddToChat(_) => "AddToChat",
            AppEvent::RemoveFromChat(_) => "RemoveFromChat",
        };
        Ok(Event::default()
            .data(serde_json::to_string(&v).expect("Failed to serialize event"))
            .event(name))
    });

    // let stream = stream::repeat_with(|| Event::default().data(format!("hi! {}", random::<u32>())))
    //     .map(Ok)
    //     .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}
