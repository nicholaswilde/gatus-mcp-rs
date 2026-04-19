use gatus_mcp_rs::client::GatusClient;
use gatus_mcp_rs::mcp::McpHandler;
use serde_json::json;

#[tokio::test]
async fn test_mcp_handle_get_badge() {
    let client = GatusClient::new("https://status.example.org".to_string(), None);
    let handler = McpHandler::new(client);

    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "get_metrics",
            "arguments": {
                "action": "get-badge",
                "id": "core_ext-ep-test"
            }
        },
        "id": 1
    });

    let response = handler.handle(request).await;
    assert!(response["error"].is_null());
    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("![Health Badge](https://status.example.org/api/v1/endpoints/core_ext-ep-test/health/badge.svg)"));
}

#[tokio::test]
async fn test_mcp_handle_get_badge_uptime() {
    let client = GatusClient::new("https://status.example.org".to_string(), None);
    let handler = McpHandler::new(client);

    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "get_metrics",
            "arguments": {
                "action": "get-badge",
                "id": "core_ext-ep-test",
                "timeframe": "24h"
            }
        },
        "id": 1
    });

    let response = handler.handle(request).await;
    assert!(response["error"].is_null());
    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("![Uptime Badge (24h)](https://status.example.org/api/v1/endpoints/core_ext-ep-test/uptimes/24h/badge.svg)"));
}
