use gatus_mcp_rs::client::GatusClient;
use gatus_mcp_rs::fmt::get_display_status;
use std::env;

#[tokio::test]
async fn test_live_list_services() {
    dotenvy::dotenv().ok();

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
            get_display_status(service),
            service.group
        );
    }
}
