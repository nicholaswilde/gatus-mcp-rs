use chrono::Utc;
use gatus_mcp_rs::client::GatusClient;
use gatus_mcp_rs::mcp::McpHandler;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_mcp_get_metrics_service_details_detailed_conditions() {
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
                    "timestamp": Utc::now().to_rfc3339(),
                    "success": false,
                    "duration": 123,
                    "errors": ["Connection reset"],
                    "conditionResults": [
                        {
                            "condition": "[STATUS] == 200",
                            "success": false
                        },
                        {
                            "condition": "[RESPONSE_TIME] < 500",
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
            "name": "get_metrics",
            "arguments": {
                "action": "service-details",
                "id": "service-1"
            }
        },
        "id": 1
    });

    let response = handler.handle(request).await;
    let text = response["result"]["content"][0]["text"].as_str().unwrap();

    assert!(text.contains("#### Conditions"));
    assert!(text.contains("❌ [STATUS] == 200"));
    assert!(text.contains("✅ [RESPONSE_TIME] < 500"));
    assert!(text.contains("- **Errors:**"));
    assert!(text.contains("Connection reset"));
}
