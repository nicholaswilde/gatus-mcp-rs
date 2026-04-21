use gatus_mcp_rs::client::GatusClient;
use gatus_mcp_rs::mcp::McpHandler;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_integration_alerts_full_flow() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);
    let handler = McpHandler::new(client);

    // 1. Test get-alert-rules
    let config_response = json!({
        "endpoints": [
            {
                "name": "api-1",
                "group": "prod",
                "url": "https://api.example.com",
                "conditions": ["[STATUS] == 200"],
                "alerts": [
                    {
                        "type": "slack",
                        "enabled": true,
                        "failure-threshold": 3,
                        "success-threshold": 2,
                        "description": "API is down"
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

    let get_rules_request = json!({
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

    let response = handler.handle(get_rules_request).await;
    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("prod"));
    assert!(text.contains("api-1"));
    assert!(text.contains("slack"));

    // 2. Test test_alert
    Mock::given(method("POST"))
        .and(path("/api/v1/endpoints/prod_api-1/test-alert"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let test_alert_request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "test_alert",
            "arguments": {
                "id": "prod_api-1"
            }
        },
        "id": 2
    });

    let response = handler.handle(test_alert_request).await;
    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("Successfully triggered test alert for 'prod_api-1'"));
}
