use gatus_mcp_rs::client::GatusClient;
use gatus_mcp_rs::mcp::McpHandler;
use serde_json::json;

#[tokio::test]
async fn test_mcp_handle_get_latency_badge() {
    let client = GatusClient::new("https://status.example.org".to_string(), None, None, None);
    let handler = McpHandler::new(client);

    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "get_metrics",
            "arguments": {
                "action": "get-latency-badge",
                "id": "core_service-1",
                "timeframe": "24h"
            }
        },
        "id": 1
    });

    let response = handler.handle(request).await;
    assert!(response["error"].is_null());
    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("![Latency Badge (24h)](https://status.example.org/api/v1/endpoints/core_service-1/response-times/24h/badge.svg)"));
}

#[tokio::test]
async fn test_mcp_handle_get_latency_chart() {
    let client = GatusClient::new("https://status.example.org".to_string(), None, None, None);
    let handler = McpHandler::new(client);

    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "get_metrics",
            "arguments": {
                "action": "get-latency-chart",
                "id": "core_service-1",
                "timeframe": "7d"
            }
        },
        "id": 1
    });

    let response = handler.handle(request).await;
    assert!(response["error"].is_null());
    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("![Latency Chart (7d)](https://status.example.org/api/v1/endpoints/core_service-1/response-times/7d/chart.svg)"));
}
