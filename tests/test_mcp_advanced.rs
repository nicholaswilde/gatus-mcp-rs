use gatus_mcp_rs::client::GatusClient;
use gatus_mcp_rs::mcp::McpHandler;
use serde_json::json;
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

#[tokio::test]
async fn test_mcp_performance_comparison() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);
    let handler = McpHandler::new(client);

    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/core_service-1/response-times/1h"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!([{"timestamp": "...", "value": 100}])),
        )
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/core_service-1/response-times/7d"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!([{"timestamp": "...", "value": 50}])),
        )
        .mount(&mock_server)
        .await;

    let response = handler
        .handle(json!({
            "jsonrpc": "2.0",
            "method": "tools/call",
            "params": {
                "name": "get_metrics",
                "arguments": { "action": "performance-comparison", "id": "core_service-1" }
            },
            "id": 1
        }))
        .await;

    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("Performance Comparison for core_service-1"));
    assert!(text.contains("+100.00%"));
}

#[tokio::test]
async fn test_mcp_alert_correlation() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);
    let handler = McpHandler::new(client);

    let gatus_response = json!([
        {
            "name": "service-1",
            "group": "core",
            "status": "UP",
            "results": [{"success": true, "timestamp": "2026-04-19T00:10:00Z", "duration": 100}],
            "events": [{"type": "ALERT", "timestamp": "2026-04-19T00:05:00Z"}]
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
                "arguments": { "action": "alert-correlation", "id": "core_service-1" }
            },
            "id": 1
        }))
        .await;

    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("Notification Correlation Timeline"));
    assert!(text.contains("🔔 alert"));
}

#[tokio::test]
async fn test_mcp_flapping_services() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);
    let handler = McpHandler::new(client);

    let metrics_response = "gatus_results_total{group=\"core\",name=\"service-1\",success=\"false\",type=\"HTTP\"} 50\n";

    Mock::given(method("GET"))
        .and(path("/metrics"))
        .respond_with(ResponseTemplate::new(200).set_body_string(metrics_response))
        .mount(&mock_server)
        .await;

    let response = handler
        .handle(json!({
            "jsonrpc": "2.0",
            "method": "tools/call",
            "params": {
                "name": "get_metrics",
                "arguments": { "action": "flapping-services" }
            },
            "id": 1
        }))
        .await;

    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("Services with Failures (Flapping)"));
    assert!(text.contains("service-1"));
}
