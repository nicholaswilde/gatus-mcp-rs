use gatus_mcp_rs::client::GatusClient;
use gatus_mcp_rs::mcp::McpHandler;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_mcp_get_system_stats() {
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
            "group": "core",
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
            "name": "get_system_stats",
            "arguments": {}
        },
        "id": 1
    });

    let response = handler.handle(request).await;

    let text = response["result"]["content"][0]["text"].as_str().expect("Response should have text content");
    assert!(text.contains("Total Endpoints:** 2"));
    assert!(text.contains("UP:** 1"));
    assert!(text.contains("DOWN:** 1"));
}
