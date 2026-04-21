use gatus_mcp_rs::client::GatusClient;
use gatus_mcp_rs::mcp::McpHandler;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_mcp_handle_get_alert_rules() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);
    let handler = McpHandler::new(client);

    let config_response = json!({
        "endpoints": [
            {
                "name": "service-1",
                "group": "core",
                "url": "http://localhost:8080",
                "conditions": ["[STATUS] == 200"],
                "alerts": [
                    {
                        "type": "slack",
                        "enabled": true,
                        "failure-threshold": 3,
                        "success-threshold": 2,
                        "description": "Service 1 is down"
                    }
                ]
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path("/api/v1/config"))
        .respond_with(ResponseTemplate::new(200).set_body_json(config_response))
        .mount(&mock_server)
        .await;

    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "manage_resources",
            "arguments": {
                "action": "get-alert-rules"
            }
        },
        "id": 1
    });

    let response = handler.handle(request).await;
    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("Gatus Alerting Rules"));
    assert!(text.contains("service-1"));
    assert!(text.contains("slack"));
}

#[tokio::test]
async fn test_mcp_handle_test_alert() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);
    let handler = McpHandler::new(client);

    Mock::given(method("POST"))
        .and(path("/api/v1/endpoints/core_service-1/test-alert"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "test_alert",
            "arguments": {
                "id": "core_service-1"
            }
        },
        "id": 1
    });

    let response = handler.handle(request).await;
    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("Successfully triggered test alert for 'core_service-1'"));
}
