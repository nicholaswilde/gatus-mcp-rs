use gatus_mcp_rs::client::GatusClient;
use gatus_mcp_rs::mcp::McpHandler;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_mcp_handle_get_suite_health() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);
    let handler = McpHandler::new(client);

    let gatus_response = json!({
        "key": "page-1",
        "name": "Main Suite",
        "results": [
            {
                "endpointResults": [
                    { "success": true },
                    { "success": true },
                    { "success": false }
                ]
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path("/api/v1/suites/page-1/statuses"))
        .respond_with(ResponseTemplate::new(200).set_body_json(gatus_response))
        .mount(&mock_server)
        .await;

    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "manage_resources",
            "arguments": {
                "action": "get-suite-health",
                "id": "page-1"
            }
        },
        "id": 1
    });

    let response = handler.handle(request).await;
    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("Suite Health: Main Suite (page-1)"));
    assert!(text.contains("**UP:** 2"));
    assert!(text.contains("**DOWN:** 1"));
    assert!(text.contains("66.67%"));
}
