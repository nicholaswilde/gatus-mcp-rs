use gatus_mcp_rs::client::GatusClient;
use gatus_mcp_rs::mcp::McpHandler;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_mcp_handler_api_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/statuses"))
        .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
        .mount(&mock_server)
        .await;

    let client = GatusClient::new(mock_server.uri(), None);
    let handler = McpHandler::new(client);

    let req = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "manage_services",
            "arguments": {
                "action": "list"
            }
        },
        "id": 1
    });

    let resp = handler.handle(req).await;

    assert_eq!(resp["id"], 1);
    assert_eq!(resp["error"]["code"], -32000);
    assert!(resp["error"]["message"]
        .as_str()
        .unwrap()
        .contains("Gatus API error"));
}

#[tokio::test]
async fn test_mcp_handler_service_not_found() {
    let mock_server = MockServer::start().await;

    // Return empty list
    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/statuses"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
        .mount(&mock_server)
        .await;

    let client = GatusClient::new(mock_server.uri(), None);
    let handler = McpHandler::new(client);

    let req = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "get_service_info",
            "arguments": {
                "service": "non-existent",
                "action": "details"
            }
        },
        "id": 1
    });

    let resp = handler.handle(req).await;

    assert_eq!(resp["id"], 1);
    assert_eq!(resp["error"]["code"], -32602);
    assert!(resp["error"]["message"]
        .as_str()
        .unwrap()
        .contains("Service 'non-existent' not found"));
}
