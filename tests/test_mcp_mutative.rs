use gatus_mcp_rs::client::GatusClient;
use gatus_mcp_rs::mcp::McpHandler;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_handle_call_trigger_check() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);
    let handler = McpHandler::new(client);

    Mock::given(method("POST"))
        .and(path("/api/v1/endpoints/core_service-1/trigger"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "trigger_check",
            "arguments": {
                "id": "core_service-1"
            }
        },
        "id": 1
    });

    let response = handler.handle(request).await;
    assert_eq!(response["id"], 1);
    assert!(response.get("result").is_some());
}

#[tokio::test]
async fn test_handle_call_reload_config() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);
    let handler = McpHandler::new(client);

    Mock::given(method("POST"))
        .and(path("/api/v1/config/reload"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "reload_config",
            "arguments": {}
        },
        "id": 1
    });

    let response = handler.handle(request).await;
    assert_eq!(response["id"], 1);
    assert!(response.get("result").is_some());
}
