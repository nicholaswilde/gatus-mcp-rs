use gatus_mcp_rs::client::GatusClient;
use gatus_mcp_rs::mcp::McpHandler;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_mcp_get_alert_history() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None);
    let handler = McpHandler::new(client);

    let gatus_response = json!([
        {
            "name": "service-1",
            "group": "core",
            "status": "UP",
            "results": [],
            "events": [
                {
                    "type": "HEALTHY",
                    "timestamp": "2024-04-11T08:45:00Z"
                },
                {
                    "type": "UNHEALTHY",
                    "timestamp": "2024-04-11T08:40:00Z"
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
            "name": "get_alert_history",
            "arguments": {
                "limit": 1
            }
        },
        "id": 1
    });

    let response = handler.handle(request).await;

    let result = &response["result"]["content"][0]["text"];
    assert!(result.as_str().unwrap().contains("service-1"));
    assert!(result.as_str().unwrap().contains("HEALTHY"));
    assert!(result.as_str().unwrap().contains("2024-04-11T08:45:00Z"));
}
