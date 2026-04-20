use gatus_mcp_rs::client::GatusClient;
use gatus_mcp_rs::mcp::McpHandler;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_mcp_handle_list_status_pages() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);
    let handler = McpHandler::new(client);

    let gatus_response = json!([
        {
            "id": "page-1",
            "name": "Main Page"
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v1/external/status-pages"))
        .respond_with(ResponseTemplate::new(200).set_body_json(gatus_response))
        .mount(&mock_server)
        .await;

    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "manage_endpoints",
            "arguments": {
                "action": "list-status-pages"
            }
        },
        "id": 1
    });

    let response = handler.handle(request).await;
    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("| page-1 | Main Page |"));
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
                "status_page_id": "page-1",
                "config": {
                    "name": "new-svc",
                    "url": "http://example.com",
                    "conditions": ["[STATUS] == 200"]
                }
            }
        },
        "id": 1
    });

    let response = handler.handle(request).await;
    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("Successfully created endpoint"));
}
