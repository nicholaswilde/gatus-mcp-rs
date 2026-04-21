use gatus_mcp_rs::client::GatusClient;
use std::env;

#[tokio::test]
async fn test_live_list_suites() {
    dotenvy::dotenv().ok();

    if env::var("GATUS_LIVE_TESTS").unwrap_or_default() != "true" {
        println!("Skipping live test: GATUS_LIVE_TESTS is not set to 'true'");
        return;
    }

    let api_url = env::var("GATUS_API_URL").expect("GATUS_API_URL must be set for live tests");
    let api_key = env::var("GATUS_API_KEY").ok();

    println!("Testing list_suites against live instance: {}", api_url);

    let client = GatusClient::new(api_url, api_key, None, None);

    let suites = client.list_suites().await.expect("Failed to list suites");
    println!("Found {} suites", suites.len());

    for suite in &suites {
        println!("Suite: {} (ID: {})", suite.name, suite.id);
    }

    if let Some(suite) = suites.first() {
        println!("Testing get_suite_health for: {}", suite.id);
        let health = client
            .get_suite_health(&suite.id)
            .await
            .expect("Failed to get suite health");
        println!(
            "Suite: {}, UP: {}, DOWN: {}",
            health.name, health.up, health.down
        );
        assert_eq!(health.id, suite.id);
    } else {
        println!("No suites found to test health.");
    }
}
