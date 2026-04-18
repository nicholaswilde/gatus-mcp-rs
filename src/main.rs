use clap::Parser;
use gatus_mcp_rs::cli::Cli;
use gatus_mcp_rs::run_app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    run_app(cli).await
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_main_not_crashing() {
        // This won't actually call main because it's #[tokio::main]
        // But we can check if it compiles.
    }
}
