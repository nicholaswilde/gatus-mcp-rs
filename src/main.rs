use clap::Parser;
use gatus_mcp_rs::http_server::app;
use gatus_mcp_rs::settings::Settings;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Gatus API base URL (e.g. http://192.168.2.220:8080)
    #[arg(long, env = "GATUS_API_URL")]
    gatus_url: Option<String>,

    /// Port to listen on
    #[arg(short, long, env = "GATUS_SERVER_PORT")]
    port: Option<u16>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Parse command line arguments
    let args = Args::parse();

    // Load settings from config files and env
    let mut settings = Settings::new()?;

    // Override settings with command line arguments if provided
    if let Some(url) = args.gatus_url {
        settings.gatus.api_url = url;
    }
    if let Some(port) = args.port {
        settings.server.port = port;
    }

    let addr = SocketAddr::from(([127, 0, 0, 1], settings.server.port));
    
    let app = app(settings);
    
    tracing::info!("Listening on {}", addr);
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
