use gatus_mcp_rs::client::GatusClient;
use std::env;

#[tokio::test]
async fn test_live_group_summary() {
    dotenvy::dotenv().ok();

    if env::var("GATUS_LIVE_TESTS").unwrap_or_default() != "true" {
        println!("Skipping live test: GATUS_LIVE_TESTS is not set to 'true'");
        return;
    }

    let api_url = env::var("GATUS_API_URL").expect("GATUS_API_URL must be set for live tests");
    let api_key = env::var("GATUS_API_KEY").ok();

    let client = GatusClient::new(api_url, api_key);
    let services = client
        .list_services()
        .await
        .expect("Failed to list services from live instance");

    if let Some(service) = services.first() {
        let group = &service.group;
        println!("Testing group summary for group: {}", group);

        let filtered: Vec<_> = services
            .iter()
            .filter(|s| s.group.to_lowercase() == group.to_lowercase())
            .collect();

        assert!(
            !filtered.is_empty(),
            "Group should have at least one service"
        );
        println!("Found {} services in group '{}'", filtered.len(), group);
    } else {
        println!("No services found on live instance, skipping.");
    }
}
