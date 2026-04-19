use gatus_mcp_rs::client::GatusClient;
use gatus_mcp_rs::mcp::McpHandler;
use serde_json::json;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_mcp_handle_push_result() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), Some("test-key".to_string(, None, None)));
    let handler = McpHandler::new(client);

    Mock::given(method("POST"))
        .and(path("/api/v1/endpoints/test-endpoint/external"))
        .and(query_param("success", "true"))
        .and(query_param("duration", "123"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "push_result",
            "arguments": {
                "id": "test-endpoint",
                "success": true,
                "duration": 123
            }
        },
        "id": 1
    });

    let response = handler.handle(request).await;
    assert!(response["error"].is_null());
    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("Successfully pushed result for 'test-endpoint'"));
}

#[tokio::test]
async fn test_mcp_handle_push_result_error() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);
    let handler = McpHandler::new(client);

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(404).set_body_string("not found"))
        .mount(&mock_server)
        .await;

    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "push_result",
            "arguments": {
                "id": "unknown",
                "success": false
            }
        },
        "id": 1
    });

    let response = handler.handle(request).await;
    assert_eq!(response["error"]["code"], -32000);
    assert!(response["error"]["message"]
        .as_str()
        .unwrap()
        .contains("Error pushing result"));
}
