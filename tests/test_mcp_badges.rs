use gatus_mcp_rs::client::GatusClient;
use gatus_mcp_rs::mcp::McpHandler;
use serde_json::json;

#[tokio::test]
async fn test_mcp_handle_get_badge() {
    let client = GatusClient::new("https://status.example.org".to_string(), None, None, None);
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
    let client = GatusClient::new("https://status.example.org".to_string(), None, None, None);
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

#[tokio::test]
async fn test_mcp_handle_service_details_with_badge() {
    let mock_server = wiremock::MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);
    let handler = McpHandler::new(client);

    let gatus_response = json!([
        {
            "name": "service-1",
            "group": "core",
            "status": "UP",
            "results": []
        }
    ]);

    wiremock::Mock::given(wiremock::matchers::method("GET"))
        .and(wiremock::matchers::path("/api/v1/endpoints/statuses"))
        .respond_with(wiremock::ResponseTemplate::new(200).set_body_json(gatus_response))
        .mount(&mock_server)
        .await;

    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "get_metrics",
            "arguments": {
                "action": "service-details",
                "id": "service-1"
            }
        },
        "id": 1
    });

    let response = handler.handle(request).await;
    assert!(response["error"].is_null());
    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("### service-1"));
    assert!(text.contains("![Health Badge]("));
    assert!(text.contains("/api/v1/endpoints/core_service-1/health/badge.svg)"));
}
