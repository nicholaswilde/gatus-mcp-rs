use gatus_mcp_rs::client::GatusClient;
use gatus_mcp_rs::mcp::McpHandler;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_mcp_get_metrics_system_stats() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None);
    let handler = McpHandler::new(client);

    let gatus_response = json!([
        { "name": "s1", "group": "g1", "status": "UP", "results": [{"timestamp": "2026-04-11T12:00:00Z", "success": true, "duration": 123, "errors": [] }] },
        { "name": "s2", "group": "g2", "status": "DOWN", "results": [{"timestamp": "2026-04-11T12:00:00Z", "success": false, "duration": 456, "errors": [] }] }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/statuses"))
        .respond_with(ResponseTemplate::new(200).set_body_json(gatus_response))
        .mount(&mock_server)
        .await;

    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "get_metrics",
            "arguments": {
                "action": "system-stats"
            }
        },
        "id": 1
    });

    let response = handler.handle(request).await;
    assert!(
        response["error"].is_null(),
        "Expected no error, got: {:?}",
        response["error"]
    );
    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    println!("TEXT:\n{}", text);
    assert!(text.contains("**Total Endpoints:** 2"));
    assert!(text.contains("**UP:** 1"));
    assert!(text.contains("**DOWN:** 1"));
}

#[tokio::test]
async fn test_mcp_get_metrics_service_details() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None);
    let handler = McpHandler::new(client);

    let gatus_response = json!([
        {
            "name": "service-1",
            "group": "core",
            "status": "UP",
            "results": [
                {
                    "timestamp": "2026-04-11T12:00:00Z",
                    "success": true,
                    "duration": 123,
                    "errors": []
                }
            ]
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/statuses"))
        .respond_with(ResponseTemplate::new(200).set_body_json(gatus_response))
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
    assert!(
        response["error"].is_null(),
        "Expected no error, got: {:?}",
        response["error"]
    );
    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("### service-1"));
    assert!(text.contains("- **Group:** core"));
    assert!(text.contains("- **Status:** UP"));
}
