use gatus_mcp_rs::client::{GatusClient, HealthResult};
use std::env;
use chrono::Utc;

#[tokio::test]
async fn test_live_push_endpoint_result() {
    dotenvy::dotenv().ok();

    if env::var("GATUS_LIVE_TESTS").unwrap_or_default() != "true" {
        println!("Skipping live test: GATUS_LIVE_TESTS is not set to 'true'");
        return;
    }

    let api_url = env::var("GATUS_API_URL").expect("GATUS_API_URL must be set for live tests");
    let api_key = env::var("GATUS_API_KEY").ok();

    println!("Testing push_endpoint_result against live instance: {}", api_url);

    let client = GatusClient::new(api_url, api_key, None, None);

    // Use a dummy key for testing, or a real one if found
    let services = client.list_services(false).await.expect("Failed to list services");
    
    let key = if let Some(service) = services.first() {
        service.name.clone()
    } else {
        "mcp-test-push".to_string()
    };

    println!("Pushing result for key: {}", key);

    let result = HealthResult {
        timestamp: Utc::now().to_rfc3339(),
        success: true,
        hostname: Some("mcp-test-host".to_string()),
        ip: Some("127.0.0.1".to_string()),
        duration: 123,
        errors: vec![],
        status: Some(200),
        condition_results: vec![],
        body: Some("Pushed from MCP live test".to_string()),
        headers: None,
        certificate_expiration: None,
    };

    let push_result = client.push_endpoint_result(&key, result).await;
    
    match push_result {
        Ok(_) => println!("Successfully pushed result to {}", key),
        Err(e) => {
            println!("Failed to push result to {}: {}", key, e);
            // Some Gatus instances might return 404 if the key is not found
            // or 401 if the API key doesn't have write permissions.
            // For the purpose of this live test, we'll see what happens.
            panic!("Live push failed: {}", e);
        }
    }
}
