use gatus_mcp_rs::client::GatusClient;
use std::env;

#[tokio::test]
async fn test_live_list_services() {
    dotenvy::dotenv().ok();

    if env::var("GATUS_LIVE_TESTS").unwrap_or_default() != "true" {
        println!("Skipping live test: GATUS_LIVE_TESTS is not set to 'true'");
        return;
    }

    let api_url = env::var("GATUS_API_URL").expect("GATUS_API_URL must be set for live tests");
    let api_key = env::var("GATUS_API_KEY").ok();

    println!("Testing against live instance: {}", api_url);

    let client = GatusClient::new(api_url, api_key);
    let services = client
        .list_services()
        .await
        .expect("Failed to list services from live instance");

    assert!(
        !services.is_empty(),
        "Live instance should have at least one service"
    );
    println!("Found {} services", services.len());

    for service in services.iter().take(5) {
        println!(
            "- {}: {} (Group: {})",
            service.name,
            service.display_status(),
            service.group
        );
    }
}
