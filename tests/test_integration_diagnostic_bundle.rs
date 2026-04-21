use gatus_mcp_rs::client::GatusClient;
use gatus_mcp_rs::mcp::McpHandler;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_integration_diagnostic_bundle_full_flow() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);
    let handler = McpHandler::new(client);

    let health_response = json!([
        {
            "name": "api-service",
            "group": "production",
            "results": [
                {
                    "timestamp": "2023-01-01T12:00:00Z",
                    "success": false,
                    "errors": ["timeout"],
                    "duration": 5000000000u64,
                    "conditionResults": [
                        {
                            "condition": "[STATUS] == 200",
                            "success": false
                        }
                    ]
                }
            ],
            "events": [
                {
                    "type": "alert",
                    "timestamp": "2023-01-01T12:00:05Z"
                }
            ]
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/production_api-service/statuses"))
        .respond_with(ResponseTemplate::new(200).set_body_json(health_response))
        .mount(&mock_server)
        .await;

    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "get_metrics",
            "arguments": {
                "action": "diagnostic-bundle",
                "id": "production_api-service"
            }
        },
        "id": "test-1"
    });

    let response = handler.handle(request).await;
    let text = response["result"]["content"][0]["text"].as_str().unwrap();

    assert!(text.contains("Diagnostic Bundle: api-service (production)"));
    assert!(text.contains("#### ❌ Failed Conditions"));
    assert!(text.contains("[STATUS] == 200"));
    assert!(text.contains("#### 📊 Recent Results"));
    assert!(text.contains("5000ms"));
    assert!(text.contains("#### 🔔 Recent Alert Events"));
    assert!(text.contains("alert"));
}
