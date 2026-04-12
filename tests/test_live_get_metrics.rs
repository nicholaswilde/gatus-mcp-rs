use gatus_mcp_rs::client::GatusClient;
use std::env;

#[tokio::test]
async fn test_live_get_metrics() {
    dotenvy::dotenv().ok();

    if env::var("GATUS_LIVE_TESTS").unwrap_or_default() != "true" {
        println!("Skipping live test: GATUS_LIVE_TESTS is not set to 'true'");
        return;
    }

    let api_url = env::var("GATUS_API_URL").expect("GATUS_API_URL must be set for live tests");
    let api_key = env::var("GATUS_API_KEY").ok();

    println!("Testing get_metrics against live instance: {}", api_url);

    let client = GatusClient::new(api_url, api_key);

    // System stats test
    let stats = client
        .get_system_stats()
        .await
        .expect("Failed to get system stats from live instance");
    println!(
        "System stats: Total: {}, UP: {}, DOWN: {}, DEGRADED: {}",
        stats.total, stats.up, stats.down, stats.degraded
    );

    // Get list of services to fetch details for
    let services = client.list_services().await.unwrap();
    if let Some(service) = services.first() {
        println!("Service Details for {}", service.name);
        println!("  - Group: {}", service.group);
        println!("  - Status: {}", service.display_status());

        // Alert History
        let alerts = client.get_alert_history(2).await.unwrap_or_default();
        println!("Alert History (limit 2): {} alerts found.", alerts.len());
    } else {
        println!("No services found to test details against.");
    }
}
