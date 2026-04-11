use gatus_mcp_rs::client::GatusClient;
use gatus_mcp_rs::mcp::McpHandler;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_mcp_get_uptime() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None);
    let handler = McpHandler::new(client);

    let gatus_response = json!([
        {
            "name": "service-1",
            "group": "core",
            "status": "UP",
            "results": [
                {
                    "timestamp": "2026-04-11T12:00:00Z",
                    "success": true,
                    "duration": 100
                },
                {
                    "timestamp": "2026-04-11T12:01:00Z",
                    "success": false,
                    "duration": 100
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
            "name": "get_uptime",
            "arguments": {
                "service": "service-1",
                "timeframe": "24h"
            }
        },
        "id": 1
    });

    let response = handler.handle(request).await;

    assert_eq!(
        response["result"]["content"][0]["text"],
        "Uptime for service-1 (24h): 50.00%"
    );
}
