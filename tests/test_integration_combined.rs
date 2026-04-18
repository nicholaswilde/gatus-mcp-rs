use gatus_mcp_rs::client::GatusClient;
use gatus_mcp_rs::mcp::McpHandler;
use serde_json::json;
use std::env;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Test manage_resources using Wiremock
#[tokio::test]
async fn test_integration_manage_resources_mock() {
    let mock_server = MockServer::start().await;

    // Mock for list-services
    let services_json = json!([
        {
            "name": "mock-service",
            "group": "mock-group",
            "status": "UP",
            "results": [
                {
                    "timestamp": "2026-04-18T10:00:00Z",
                    "success": true,
                    "duration": 50000000,
                    "conditionResults": []
                }
            ]
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/statuses"))
        .respond_with(ResponseTemplate::new(200).set_body_json(services_json))
        .mount(&mock_server)
        .await;

    // Mock for get-health
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(ResponseTemplate::new(200).set_body_string("UP"))
        .mount(&mock_server)
        .await;

    let client = GatusClient::new(mock_server.uri(), None);
    let handler = McpHandler::new(client);

    // 1. list-services
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
    assert!(response["error"].is_null());
    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("mock-service"));

    // 2. get-health
    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "manage_resources",
            "arguments": {
                "action": "get-health"
            }
        },
        "id": 2
    });

    let response = handler.handle(request).await;
    assert!(response["error"].is_null());
    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("UP"));
}

/// Test manage_resources using live instance (if enabled)
#[tokio::test]
async fn test_integration_manage_resources_live() {
    dotenvy::dotenv().ok();

    if env::var("GATUS_LIVE_TESTS").unwrap_or_default() != "true" {
        return;
    }

    let api_url = env::var("GATUS_API_URL").expect("GATUS_API_URL must be set for live tests");
    let api_key = env::var("GATUS_API_KEY").ok();

    let client = GatusClient::new(api_url, api_key);
    let handler = McpHandler::new(client);

    // 1. list-services
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
        "Live list-services failed: {:?}",
        response["error"]
    );
    assert!(response["result"]["content"][0]["text"].as_str().is_some());

    // 2. get-health
    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "manage_resources",
            "arguments": {
                "action": "get-health"
            }
        },
        "id": 2
    });

    let response = handler.handle(request).await;
    assert!(
        response["error"].is_null(),
        "Live get-health failed: {:?}",
        response["error"]
    );
}

/// Combined integration test for get_metrics
#[tokio::test]
async fn test_integration_get_metrics_mock() {
    let mock_server = MockServer::start().await;

    // Return system stats data
    let services_json = json!([
        {
            "name": "svc1",
            "group": "grp1",
            "status": "UP",
            "results": [
                {
                    "timestamp": "2026-04-18T10:00:00Z",
                    "success": true,
                    "duration": 50000000,
                    "conditionResults": []
                }
            ]
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/statuses"))
        .respond_with(ResponseTemplate::new(200).set_body_json(services_json))
        .mount(&mock_server)
        .await;

    let client = GatusClient::new(mock_server.uri(), None);
    let handler = McpHandler::new(client);

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
    assert!(response["error"].is_null());
    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("**Total Endpoints:** 1"));
}

/// Test prompts/get integration
#[tokio::test]
async fn test_integration_prompts_get_mock() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None);
    let handler = McpHandler::new(client);

    let request = json!({
        "jsonrpc": "2.0",
        "method": "prompts/get",
        "params": {
            "name": "analyze-outage",
            "arguments": {
                "id": "my-service"
            }
        },
        "id": 1
    });

    let response = handler.handle(request).await;
    assert!(response["error"].is_null());
    let messages = response["result"]["messages"]
        .as_array()
        .expect("messages should be an array");
    assert!(messages[0]["content"]["text"]
        .as_str()
        .unwrap()
        .contains("my-service"));
}
