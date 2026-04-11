use gatus_mcp_rs::settings::Settings;
use gatus_mcp_rs::http_server::app;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let settings = Settings::new()?;
    let addr = SocketAddr::from(([127, 0, 0, 1], settings.server.port));
    
    let app = app(settings);
    
    tracing::info!("Listening on {}", addr);
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
