use chrono::Utc;
use gatus_mcp_rs::client::GatusClient;
use gatus_mcp_rs::mcp::McpHandler;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_mcp_get_metrics_system_stats() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);
    let handler = McpHandler::new(client);

    let gatus_response = json!([
        { "name": "s1", "group": "g1", "status": "UP", "results": [{"timestamp": Utc::now().to_rfc3339(), "success": true, "duration": 123, "errors": [] }] },
        { "name": "s2", "group": "g2", "status": "DOWN", "results": [{"timestamp": Utc::now().to_rfc3339(), "success": false, "duration": 456, "errors": [] }] }
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
    assert!(text.contains("**Total Endpoints:** 2"));
    assert!(text.contains("**UP:** 1"));
    assert!(text.contains("**DOWN:** 1"));
}

#[tokio::test]
async fn test_mcp_get_metrics_service_details() {
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
                    "timestamp": Utc::now().to_rfc3339(),
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

#[tokio::test]
async fn test_mcp_get_metrics_service_history() {
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
                    "timestamp": Utc::now().to_rfc3339(),
                    "success": true,
                    "duration": 123,
                    "errors": []
                },
                {
                    "timestamp": (Utc::now() - chrono::Duration::minutes(1)).to_rfc3339(),
                    "success": false,
                    "duration": 456,
                    "errors": ["Connection reset"]
                }
            ]
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/service-1/statuses"))
        .respond_with(ResponseTemplate::new(200).set_body_json(gatus_response))
        .mount(&mock_server)
        .await;

    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "get_metrics",
            "arguments": {
                "action": "service-history",
                "id": "service-1",
                "limit": 2
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
    // It returns pretty-printed JSON array
    assert!(text.contains("\"success\": true"));
    assert!(text.contains("\"success\": false"));
    assert!(text.contains("Connection reset"));
}

#[tokio::test]
async fn test_mcp_get_metrics_group_summary() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);
    let handler = McpHandler::new(client);

    let gatus_response = json!([
        { "name": "s1", "group": "media", "status": "UP", "results": [] },
        { "name": "s2", "group": "media", "status": "DOWN", "results": [] },
        { "name": "s3", "group": "core", "status": "UP", "results": [] }
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
                "action": "group-summary",
                "id": "media"
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
    // Returns format_endpoints_summary table
    assert!(text.contains("| Service | Group | Status | Latest Result |"));
    assert!(text.contains("s1"));
    assert!(text.contains("s2"));
    assert!(!text.contains("s3"));
}

#[tokio::test]
async fn test_mcp_get_metrics_uptime() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);
    let handler = McpHandler::new(client);

    let now = Utc::now();
    let ts1 = (now - chrono::Duration::minutes(10)).to_rfc3339();
    let ts2 = (now - chrono::Duration::minutes(5)).to_rfc3339();

    let gatus_response = json!([
        {
            "name": "service-1",
            "group": "core",
            "results": [
                { "timestamp": ts1, "success": true, "duration": 100 },
                { "timestamp": ts2, "success": false, "duration": 100 }
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
                "action": "uptime",
                "id": "service-1",
                "timeframe": "24h"
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
    assert!(text.contains("50.00%"));
}

#[tokio::test]
async fn test_mcp_get_metrics_response_time() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);
    let handler = McpHandler::new(client);

    let gatus_response = json!([
        { "timestamp": Utc::now().to_rfc3339(), "value": 150 },
        { "timestamp": (Utc::now() - chrono::Duration::minutes(1)).to_rfc3339(), "value": 200 }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/Core_Frontend/response-times/24h"))
        .respond_with(ResponseTemplate::new(200).set_body_json(gatus_response))
        .mount(&mock_server)
        .await;

    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "get_metrics",
            "arguments": {
                "action": "response-time",
                "id": "Core_Frontend"
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
    assert!(text.contains("Core_Frontend"));
    assert!(text.contains("Average: 175.00ms"));
    assert!(text.contains("Min: 150ms"));
    assert!(text.contains("Max: 200ms"));
}

#[tokio::test]
async fn test_mcp_get_metrics_alert_history() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);
    let handler = McpHandler::new(client);

    let gatus_response = json!([
        {
            "name": "service-1",
            "group": "core",
            "events": [
                { "type": "HEALTHY", "timestamp": Utc::now().to_rfc3339() },
                { "type": "UNHEALTHY", "timestamp": (Utc::now() - chrono::Duration::minutes(10)).to_rfc3339() }
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
                "action": "alert-history",
                "limit": 2
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
    assert!(text.contains("HEALTHY"));
    assert!(text.contains("UNHEALTHY"));
}
