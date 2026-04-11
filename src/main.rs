use clap::Parser;
use gatus_mcp_rs::cli::{Cli, Commands};
use gatus_mcp_rs::client::GatusClient;
use gatus_mcp_rs::mcp::McpHandler;
use gatus_mcp_rs::server::{run_http_server, run_stdio_server};
use gatus_mcp_rs::settings::Settings;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();

    // Initialize logging
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&cli.log_level));

    let registry = tracing_subscriber::registry().with(filter);

    if cli.log_format == "json" {
        registry.with(fmt::layer().json()).init();
    } else {
        registry.with(fmt::layer()).init();
    }

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
            let client = GatusClient::new(settings.gatus.api_url, settings.gatus.api_key);
            let handler = McpHandler::new(client);
            run_stdio_server(handler).await?;
        }
        Commands::Http { port, host } => {
            run_http_server(settings, port, host).await?;
        }
        Commands::ListTools => {
            let client = GatusClient::new(settings.gatus.api_url, settings.gatus.api_key);
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
            let client = GatusClient::new(settings.gatus.api_url, settings.gatus.api_key);
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
