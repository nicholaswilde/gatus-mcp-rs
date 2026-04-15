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
    sync::Arc,
    time::Duration,
};
use tokio::io::{self, AsyncBufRead, AsyncBufReadExt, AsyncWrite, AsyncWriteExt};
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
    let stdout = io::stdout();
    run_server_loop(handler, io::BufReader::new(stdin), stdout).await
}

pub async fn run_server_loop<R, W>(
    handler: McpHandler,
    mut reader: R,
    mut writer: W,
) -> anyhow::Result<()>
where
    R: AsyncBufRead + Unpin,
    W: AsyncWrite + Unpin,
{
    let mut line = String::new();

    tracing::info!("Ready to receive MCP messages");

    while reader.read_line(&mut line).await? > 0 {
        let request: Value = match serde_json::from_str(&line) {
            Ok(v) => {
                tracing::debug!("Received request: {}", v);
                v
            }
            Err(e) => {
                tracing::error!("Failed to parse JSON: {}", e);
                line.clear();
                continue;
            }
        };

        let response = handler.handle(request).await;
        let response_str = serde_json::to_string(&response)?;
        tracing::debug!("Sending response: {}", response_str);
        writer.write_all(response_str.as_bytes()).await?;
        writer.write_all(b"\n").await?;
        writer.flush().await?;
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
