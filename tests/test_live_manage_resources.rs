use gatus_mcp_rs::client::GatusClient;
use std::env;

#[tokio::test]
async fn test_live_manage_resources() {
    dotenvy::dotenv().ok();

    if env::var("GATUS_LIVE_TESTS").unwrap_or_default() != "true" {
        println!("Skipping live test: GATUS_LIVE_TESTS is not set to 'true'");
        return;
    }

    let api_url = env::var("GATUS_API_URL").expect("GATUS_API_URL must be set for live tests");
    let api_key = env::var("GATUS_API_KEY").ok();

    println!(
        "Testing manage_resources against live instance: {}",
        api_url
    );

    let client = GatusClient::new(api_url, api_key);

    // Test list_services action
    let services = client
        .list_services()
        .await
        .expect("Failed to list services from live instance for manage_resources");

    assert!(
        !services.is_empty(),
        "Live instance should have at least one service for manage_resources"
    );

    let first_group = services[0].group.clone();
    println!(
        "Found {} services. First group is {}",
        services.len(),
        first_group
    );

    // We only test the client layer here for integration because the MCP handler just wraps these calls
    let health = client
        .get_instance_health()
        .await
        .expect("Failed to get instance health");
    println!("Live instance health: {}", health);
}
