use gatus_mcp_rs::client::GatusClient;
use gatus_mcp_rs::mcp::McpHandler;
use serde_json::{json, Value};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_mcp_list_expiring_certificates() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);
    let handler = McpHandler::new(client);

    let gatus_response = json!([
        {
            "name": "service-1",
            "group": "core",
            "status": "UP",
            "results": [
                {
                    "success": true,
                    "timestamp": "2026-04-19T00:00:00Z",
                    "duration": 100,
                    "certificate_expiration": 1000000
                }
            ]
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/statuses"))
        .respond_with(ResponseTemplate::new(200).set_body_json(gatus_response))
        .mount(&mock_server)
        .await;

    let response = handler
        .handle(json!({
            "jsonrpc": "2.0",
            "method": "tools/call",
            "params": {
                "name": "manage_resources",
                "arguments": { "action": "list-expiring-certificates" }
            },
            "id": 1
        }))
        .await;

    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("| service-1 | core |"));
}

#[tokio::test]
async fn test_mcp_failure_summary() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);
    let handler = McpHandler::new(client);

    let gatus_response = json!([
        {
            "name": "service-1",
            "group": "core",
            "status": "DOWN",
            "results": [
                {
                    "success": false,
                    "timestamp": "2026-04-19T00:00:00Z",
                    "duration": 100,
                    "conditionResults": [
                        {"condition": "[STATUS] == 200", "success": false}
                    ]
                }
            ]
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/core_service-1/statuses"))
        .respond_with(ResponseTemplate::new(200).set_body_json(gatus_response))
        .mount(&mock_server)
        .await;

    let response = handler
        .handle(json!({
            "jsonrpc": "2.0",
            "method": "tools/call",
            "params": {
                "name": "get_metrics",
                "arguments": { "action": "failure-summary", "id": "core_service-1" }
            },
            "id": 1
        }))
        .await;

    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("### Failure Summary for service-1 (core)"));
    assert!(text.contains("- [STATUS] == 200"));
}

#[tokio::test]
async fn test_mcp_group_stats() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);
    let handler = McpHandler::new(client);

    let gatus_response = json!([
        {
            "name": "service-1",
            "group": "core",
            "status": "UP",
            "results": [{"success": true, "timestamp": "2026-04-19T00:00:00Z", "duration": 100}]
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/statuses"))
        .respond_with(ResponseTemplate::new(200).set_body_json(gatus_response))
        .mount(&mock_server)
        .await;

    let response = handler
        .handle(json!({
            "jsonrpc": "2.0",
            "method": "tools/call",
            "params": {
                "name": "get_metrics",
                "arguments": { "action": "group-stats", "id": "core" }
            },
            "id": 1
        }))
        .await;

    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("### Group Health: core"));
}

#[tokio::test]
async fn test_mcp_list_services_filtered() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);
    let handler = McpHandler::new(client);

    let gatus_response = json!([
        {
            "name": "service-up",
            "group": "core",
            "status": "UP",
            "results": [{"success": true, "timestamp": "2026-04-19T00:00:00Z", "duration": 100}]
        },
        {
            "name": "service-down",
            "group": "core",
            "status": "DOWN",
            "results": [{"success": false, "timestamp": "2026-04-19T00:00:00Z", "duration": 100}]
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/statuses"))
        .respond_with(ResponseTemplate::new(200).set_body_json(gatus_response))
        .mount(&mock_server)
        .await;

    let response = handler
        .handle(json!({
            "jsonrpc": "2.0",
            "method": "tools/call",
            "params": {
                "name": "manage_resources",
                "arguments": { "action": "list-services", "status": "DOWN" }
            },
            "id": 1
        }))
        .await;

    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("service-down"));
    assert!(!text.contains("service-up"));
}
