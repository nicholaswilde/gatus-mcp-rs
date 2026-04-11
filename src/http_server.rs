use crate::settings::Settings;
use axum::{
    response::sse::{Event, Sse},
    routing::{get, post},
    Router,
};
use futures::stream::Stream;
use std::{convert::Infallible, time::Duration};
use tokio_stream::StreamExt as _;

pub fn app(_settings: Settings) -> Router {
    Router::new()
        .route("/sse", get(sse_handler))
        .route("/messages", post(messages_handler))
}

async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = tokio_stream::iter(std::iter::repeat(Ok(Event::default().data("ping"))))
        .throttle(Duration::from_secs(15));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("keep-alive"),
    )
}

async fn messages_handler(body: String) -> &'static str {
    println!("Received message: {}", body);
    "OK"
}
