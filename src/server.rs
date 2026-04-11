use crate::cli::AppState;
use crate::client::GatusClient;
use crate::mcp::McpHandler;
use crate::settings::Settings;
use axum::{
    extract::State,
    response::sse::{Event, Sse},
    routing::{get, post},
    Json, Router,
};
use futures::stream::Stream;
use serde_json::Value;
use std::net::SocketAddr;
use std::{
    convert::Infallible,
    io::{self, BufRead},
    sync::Arc,
    time::Duration,
};
use tokio::net::TcpListener;
use tokio_stream::StreamExt as _;

pub fn create_app(settings: Settings) -> Router {
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

pub async fn run_stdio_server(handler: McpHandler) -> anyhow::Result<()> {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut line = String::new();

    tracing::info!("Ready to receive MCP messages on stdin");

    while reader.read_line(&mut line)? > 0 {
        let request: Value = match serde_json::from_str(&line) {
            Ok(v) => {
                tracing::debug!("Received request: {}", v);
                v
            }
            Err(e) => {
                tracing::error!("Failed to parse JSON from stdin: {}", e);
                line.clear();
                continue;
            }
        };

        let response = handler.handle(request).await;
        let response_str = serde_json::to_string(&response)?;
        tracing::debug!("Sending response: {}", response_str);
        println!("{}", response_str);
        line.clear();
    }

    Ok(())
}

pub async fn run_http_server(settings: Settings, port: u16, host: String) -> anyhow::Result<()> {
    let app = create_app(settings);
    let addr = format!("{}:{}", host, port).parse::<SocketAddr>()?;
    tracing::info!("Listening on {}", addr);
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
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
