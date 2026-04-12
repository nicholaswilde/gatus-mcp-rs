use gatus_mcp_rs::client::GatusClient;
use gatus_mcp_rs::mcp::McpHandler;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_mcp_manage_resources_list_services() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None);
    let handler = McpHandler::new(client);

    let gatus_response = json!([
        {
            "name": "service-1",
            "group": "core",
            "status": "UP",
            "results": []
        },
        {
            "name": "service-2",
            "group": "media",
            "status": "DOWN",
            "results": []
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
            "name": "manage_resources",
            "arguments": {
                "action": "list-services"
            }
        },
        "id": 1
    });

    let response = handler.handle(request).await;

    // Check if result exists
    assert!(response["error"].is_null(), "Expected no error, got: {:?}", response["error"]);

    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("service-1"));
    assert!(text.contains("service-2"));
}

#[tokio::test]
async fn test_mcp_manage_resources_list_groups() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None);
    let handler = McpHandler::new(client);

    let gatus_response = json!([
        { "name": "s1", "group": "core" },
        { "name": "s2", "group": "media" },
        { "name": "s3", "group": "core" }
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
            "name": "manage_resources",
            "arguments": {
                "action": "list-groups"
            }
        },
        "id": 1
    });

    let response = handler.handle(request).await;
    assert!(response["error"].is_null(), "Expected no error, got: {:?}", response["error"]);

    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("core"));
    assert!(text.contains("media"));
}

#[tokio::test]
async fn test_mcp_manage_resources_list_endpoints_all() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None);
    let handler = McpHandler::new(client);

    let gatus_response = json!([
        { "name": "service-1", "group": "core" },
        { "name": "service-2", "group": "media" }
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
            "name": "manage_resources",
            "arguments": {
                "action": "list-endpoints"
            }
        },
        "id": 1
    });

    let response = handler.handle(request).await;
    assert!(response["error"].is_null(), "Expected no error, got: {:?}", response["error"]);
    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("service-1"));
    assert!(text.contains("service-2"));
}

#[tokio::test]
async fn test_mcp_manage_resources_list_endpoints_filtered() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None);
    let handler = McpHandler::new(client);

    let gatus_response = json!([
        { "name": "service-1", "group": "core" },
        { "name": "service-2", "group": "media" }
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
            "name": "manage_resources",
            "arguments": {
                "action": "list-endpoints",
                "id": "core"
            }
        },
        "id": 1
    });

    let response = handler.handle(request).await;
    assert!(response["error"].is_null(), "Expected no error, got: {:?}", response["error"]);
    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("service-1"));
    assert!(!text.contains("service-2"));
}

#[tokio::test]
async fn test_mcp_manage_resources_get_config() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None);
    let handler = McpHandler::new(client);

    let gatus_response = json!([
        {
            "name": "service-1",
            "group": "core",
            "results": [
                {
                    "timestamp": "2026-04-11T12:00:00Z",
                    "success": true,
                    "duration": 123,
                    "conditionResults": [
                        {
                            "condition": "[STATUS] == 200",
                            "success": true
                        }
                    ]
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
            "name": "manage_resources",
            "arguments": {
                "action": "get-config"
            }
        },
        "id": 1
    });

    let response = handler.handle(request).await;
    assert!(response["error"].is_null(), "Expected no error, got: {:?}", response["error"]);
    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("service-1"));
    assert!(text.contains("[STATUS] == 200"));
}

#[tokio::test]
async fn test_mcp_manage_resources_get_health() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None);
    let handler = McpHandler::new(client);

    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(ResponseTemplate::new(200).set_body_string("OK"))
        .mount(&mock_server)
        .await;

    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "manage_resources",
            "arguments": {
                "action": "get-health"
            }
        },
        "id": 1
    });

    let response = handler.handle(request).await;
    assert!(response["error"].is_null(), "Expected no error, got: {:?}", response["error"]);
    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("OK"));
}
