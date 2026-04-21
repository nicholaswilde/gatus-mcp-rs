use gatus_mcp_rs::client::GatusClient;
use gatus_mcp_rs::mcp::McpHandler;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_mcp_handle_list_suites() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);
    let handler = McpHandler::new(client);

    let list_response = json!([
        { "key": "page-1", "name": "Main Page" }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v1/suites/statuses"))
        .respond_with(ResponseTemplate::new(200).set_body_json(list_response))
        .mount(&mock_server)
        .await;

    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "manage_endpoints",
            "arguments": {
                "action": "list-suites"
            }
        },
        "id": 1
    });

    let response = handler.handle(request).await;
    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("Gatus Suites"));
    assert!(text.contains("page-1"));
}

#[tokio::test]
async fn test_mcp_handle_create_endpoint() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);
    let handler = McpHandler::new(client);

    Mock::given(method("POST"))
        .and(path("/api/v1/external/status-pages/page-1/endpoints"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "manage_endpoints",
            "arguments": {
                "action": "create-endpoint",
                "suite_id": "page-1",
                "config": {
                    "name": "ep1",
                    "url": "http://localhost",
                    "conditions": ["[STATUS] == 200"]
                }
            }
        },
        "id": 1
    });

    let response = handler.handle(request).await;
    assert!(response["error"].is_null());
    assert_eq!(
        response["result"]["content"][0]["text"],
        "Successfully created endpoint"
    );
}
