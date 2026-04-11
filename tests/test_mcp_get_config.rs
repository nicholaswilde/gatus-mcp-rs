use gatus_mcp_rs::client::GatusClient;
use gatus_mcp_rs::mcp::McpHandler;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_mcp_get_config() {
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
                    "status": 200,
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
            "name": "get_config",
            "arguments": {}
        },
        "id": 1
    });

    let response = handler.handle(request).await;

    // Check if result exists (it shouldn't if tool not found)
    if response["error"].is_object() {
        panic!("Tool 'get_config' not found or error: {:?}", response["error"]);
    }

    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("service-1"));
    assert!(text.contains("core"));
    assert!(text.contains("[STATUS] == 200"));
}
