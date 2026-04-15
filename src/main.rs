use clap::Parser;
use gatus_mcp_rs::cli::Cli;
use gatus_mcp_rs::run_app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    run_app(cli).await
}
