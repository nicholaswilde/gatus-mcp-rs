use gatus_mcp_rs::mcp::McpHandler;
use gatus_mcp_rs::client::GatusClient;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};
use serde_json::json;

#[tokio::test]
async fn test_mcp_get_service_history_tool_integration() {
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
                    "timestamp": "2026-04-10T12:00:00Z",
                    "success": true,
                    "hostname": "localhost",
                    "ip": "127.0.0.1",
                    "duration": 100000000,
                    "errors": [],
                    "status": 200
                },
                {
                    "timestamp": "2026-04-10T11:59:00Z",
                    "success": false,
                    "hostname": "localhost",
                    "ip": "127.0.0.1",
                    "duration": 500000000,
                    "errors": ["timeout"],
                    "status": 504
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
            "name": "get_service_history",
            "arguments": {
                "service": "service-1",
                "limit": 1
            }
        },
        "id": 1
    });

    let response = handler.handle(request).await;
    
    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    let history: serde_json::Value = serde_json::from_str(text).unwrap();
    
    assert_eq!(history.as_array().unwrap().len(), 1);
    assert!(text.contains("2026-04-10T12:00:00Z"));
}
