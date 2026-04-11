use crate::mcp::McpHandler;
use crate::settings::Settings;
use crate::gatus::GatusClient;
use axum::{
    extract::State,
    response::sse::{Event, Sse},
    routing::{get, post},
    Json, Router,
};
use futures::stream::Stream;
use serde_json::Value;
use std::{convert::Infallible, sync::Arc, time::Duration};
use tokio_stream::StreamExt as _;

#[derive(Clone)]
pub struct AppState {
    pub mcp_handler: Arc<McpHandler>,
}

pub fn app(settings: Settings) -> Router {
    let gatus_client = GatusClient::new(settings.gatus.api_url, settings.gatus.api_key);
    let mcp_handler = McpHandler::new(gatus_client);
    let state = AppState {
        mcp_handler: Arc::new(mcp_handler),
    };

    Router::new()
        .route("/sse", get(sse_handler))
        .route("/messages", post(messages_handler))
        .with_state(state)
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

async fn messages_handler(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> Json<Value> {
    let response = state.mcp_handler.handle(payload).await;
    Json(response)
}
