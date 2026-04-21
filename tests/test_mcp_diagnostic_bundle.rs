use gatus_mcp_rs::client::GatusClient;
use gatus_mcp_rs::mcp::McpHandler;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_mcp_handle_get_diagnostic_bundle() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);
    let handler = McpHandler::new(client);

    let health_response = json!([
        {
            "name": "service-1",
            "group": "core",
            "results": [
                {
                    "timestamp": "2023-01-01T00:00:00Z",
                    "success": false,
                    "errors": ["connection refused"],
                    "duration": 100000000,
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
                    "timestamp": "2023-01-01T00:00:00Z"
                }
            ]
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/core_service-1/statuses"))
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
                "id": "core_service-1"
            }
        },
        "id": 1
    });

    let response = handler.handle(request).await;
    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("Diagnostic Bundle: service-1 (core)"));
    assert!(text.contains("connection refused"));
    assert!(text.contains("[STATUS] == 200"));
    assert!(text.contains("alert"));
}
