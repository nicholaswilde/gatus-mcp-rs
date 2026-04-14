use gatus_mcp_rs::client::GatusClient;
use gatus_mcp_rs::mcp::McpHandler;
use serde_json::json;
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
    let handler = McpHandler::new(client);

    // Test list-services action
    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "manage_resources",
            "arguments": {
                "action": "list-services"
            }
        },
        "id": 1
    });

    let response = handler.handle(request).await;
    assert!(
        response["error"].is_null(),
        "Expected no error for list-services, got: {:?}",
        response["error"]
    );
    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    assert!(!text.is_empty(), "Result text should not be empty");
    println!(
        "List services output sample: {}",
        &text[..text.len().min(100)]
    );

    // Test list-groups action
    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "manage_resources",
            "arguments": {
                "action": "list-groups"
            }
        },
        "id": 2
    });

    let response = handler.handle(request).await;
    assert!(
        response["error"].is_null(),
        "Expected no error for list-groups, got: {:?}",
        response["error"]
    );
    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("Available groups:"));

    // Test get-health action
    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "manage_resources",
            "arguments": {
                "action": "get-health"
            }
        },
        "id": 3
    });

    let response = handler.handle(request).await;
    assert!(
        response["error"].is_null(),
        "Expected no error for get-health, got: {:?}",
        response["error"]
    );
    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("UP") || text.contains("OK"));
}
