use gatus_mcp_rs::client::GatusClient;
use gatus_mcp_rs::mcp::McpHandler;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_integration_multi_status_full_flow() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);
    let handler = McpHandler::new(client);

    // 1. Mock list-suites
    let list_response = json!([
        { "key": "page-1", "name": "Main Suite" },
        { "key": "page-2", "name": "API Suite" }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v1/suites/statuses"))
        .respond_with(ResponseTemplate::new(200).set_body_json(list_response))
        .mount(&mock_server)
        .await;

    let list_request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "manage_endpoints",
            "arguments": {
                "action": "list-suites"
            }
        },
        "id": "list-1"
    });

    let response = handler.handle(list_request).await;
    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("page-1"));
    assert!(text.contains("page-2"));

    // 2. Mock get-suite-health
    let health_response = json!({
        "key": "page-2",
        "name": "API Suite",
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
        .and(path("/api/v1/suites/page-2/statuses"))
        .respond_with(ResponseTemplate::new(200).set_body_json(health_response))
        .mount(&mock_server)
        .await;

    let health_request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "manage_resources",
            "arguments": {
                "action": "get-suite-health",
                "id": "page-2"
            }
        },
        "id": "health-1"
    });

    let response = handler.handle(health_request).await;
    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("Suite Health: API Suite (page-2)"));
    assert!(text.contains("**UP:** 2"));
    assert!(text.contains("66.67%"));
}
