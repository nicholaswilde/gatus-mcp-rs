use gatus_mcp_rs::client::GatusClient;
use gatus_mcp_rs::mcp::McpHandler;
use serde_json::json;
use std::env;

#[tokio::test]
async fn test_live_service_info() {
    dotenvy::dotenv().ok();

    if env::var("GATUS_LIVE_TESTS").unwrap_or_default() != "true" {
        println!("Skipping live test: GATUS_LIVE_TESTS is not set to 'true'");
        return;
    }

    let api_url = env::var("GATUS_API_URL").expect("GATUS_API_URL must be set for live tests");
    let api_key = env::var("GATUS_API_KEY").ok();

    println!("Testing against live instance: {}", api_url);

    let client = GatusClient::new(api_url, api_key);
    let handler = McpHandler::new(client);

    // List services to find one to test with
    let list_req = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "manage_services",
            "arguments": {
                "action": "list"
            }
        },
        "id": 1
    });

    let list_resp = handler.handle(list_req).await;
    println!("List response: {}", list_resp);
    
    let content = list_resp["result"]["content"][0]["text"].as_str().expect("Failed to get text from response");
    assert!(content.contains("| Service | Group | Status |"), "Response should contain table header");

    // Extract first service name from summary if possible, or just use one we know exists from previous output
    // The previous output showed "Authentik", "CyberKeyGen", etc.
    let service_name = "Authentik"; 

    // Test details
    println!("Fetching details for {}...", service_name);
    let details_req = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "get_service_info",
            "arguments": {
                "service": service_name,
                "action": "details"
            }
        },
        "id": 2
    });
    let details_resp = handler.handle(details_req).await;
    println!("Details response: {}", details_resp);
    assert!(details_resp["error"].is_null(), "Details request should not return error");

    // Test history
    println!("Fetching history for {}...", service_name);
    let history_req = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "get_service_info",
            "arguments": {
                "service": service_name,
                "action": "history",
                "limit": 5
            }
        },
        "id": 3
    });
    let history_resp = handler.handle(history_req).await;
    println!("History response: {}", history_resp);
    assert!(history_resp["error"].is_null(), "History request should not return error");
}
