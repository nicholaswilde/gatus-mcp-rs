pub mod cli;
pub mod client;
pub mod fmt;
pub mod mcp;
pub mod server;
pub mod settings;

use crate::cli::{Cli, Commands};
use crate::client::GatusClient;
use crate::mcp::McpHandler;
use crate::server::{run_http_server, run_server_loop};
use crate::settings::Settings;
use tokio::io::{self, AsyncWrite};
use tracing_subscriber::{fmt as trace_fmt, prelude::*, EnvFilter};

pub async fn run_app(cli: Cli) -> anyhow::Result<()> {
    run_app_with_stdio(cli, io::stdin(), io::stdout()).await
}

pub async fn run_app_with_stdio<R, W>(cli: Cli, reader: R, writer: W) -> anyhow::Result<()>
where
    R: tokio::io::AsyncRead + Unpin + Send + 'static,
    W: AsyncWrite + Unpin + Send + 'static,
{
    // Initialize logging
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&cli.log_level));

    let registry = tracing_subscriber::registry().with(filter);

    let _ = if cli.log_format == "json" {
        registry.with(trace_fmt::layer().json()).try_init()
    } else {
        registry.with(trace_fmt::layer()).try_init()
    };

    // Load settings
    let mut settings = Settings::new()?;

    // Override settings with CLI flags if provided
    if let Some(url) = cli.gatus_url {
        settings.gatus.api_url = url;
    }
    if let Some(key) = cli.api_key {
        settings.gatus.api_key = Some(key);
    }

    match cli.command.unwrap_or(Commands::Stdio) {
        Commands::Stdio => {
            tracing::info!(
                "Starting gatus-mcp-rs v{} in stdio mode",
                env!("CARGO_PKG_VERSION")
            );
            tracing::info!("Using Gatus API URL: {}", settings.gatus.api_url);
            let client = GatusClient::new(
                settings.gatus.api_url,
                settings.gatus.api_key,
                settings.gatus.username,
                settings.gatus.password,
            );
            let handler = McpHandler::new(client);
            run_server_loop(handler, io::BufReader::new(reader), writer).await?;
        }
        Commands::Http { port, host } => {
            run_http_server(settings, port, host).await?;
        }
        Commands::ListTools => {
            let client = GatusClient::new(
                settings.gatus.api_url,
                settings.gatus.api_key,
                settings.gatus.username,
                settings.gatus.password,
            );
            let handler = McpHandler::new(client);
            let response = handler
                .handle(serde_json::json!({
                    "jsonrpc": "2.0",
                    "method": "tools/list",
                    "id": 1
                }))
                .await;
            println!("{}", serde_json::to_string_pretty(&response)?);
        }
        Commands::CallTool { name, arguments } => {
            let client = GatusClient::new(
                settings.gatus.api_url,
                settings.gatus.api_key,
                settings.gatus.username,
                settings.gatus.password,
            );
            let handler = McpHandler::new(client);
            let args: serde_json::Value = serde_json::from_str(&arguments)?;
            let response = handler
                .handle(serde_json::json!({
                    "jsonrpc": "2.0",
                    "method": "tools/call",
                    "params": {
                        "name": name,
                        "arguments": args
                    },
                    "id": 1
                }))
                .await;
            println!("{}", serde_json::to_string_pretty(&response)?);
        }
    }

    Ok(())
}
