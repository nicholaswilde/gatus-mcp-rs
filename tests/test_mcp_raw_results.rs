use gatus_mcp_rs::client::GatusClient;
use gatus_mcp_rs::mcp::McpHandler;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_mcp_get_metrics_get_raw_results() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);
    let handler = McpHandler::new(client);

    let mock_response = r#"[
        {
            "name": "service1",
            "group": "group1",
            "results": [
                {
                    "timestamp": "2023-01-01T00:00:00Z",
                    "success": true,
                    "duration": 100,
                    "conditionResults": [],
                    "body": "full-body-content",
                    "headers": {"content-type": "application/json"}
                }
            ],
            "events": []
        }
    ]"#;

    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/group1_service1/statuses"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(mock_response, "application/json"))
        .mount(&mock_server)
        .await;

    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "get_metrics",
            "arguments": {
                "action": "get-raw-results",
                "id": "group1_service1",
                "limit": 1
            }
        },
        "id": 1
    });

    let response = handler.handle(request).await;
    
    // This should fail initially because 'get-raw-results' action is not handled
    assert!(response["result"]["content"][0]["text"].as_str().is_some());
    let result_text = response["result"]["content"][0]["text"].as_str().unwrap();
    let history: serde_json::Value = serde_json::from_str(result_text).unwrap();

    assert_eq!(history.as_array().unwrap().len(), 1);
    assert_eq!(history[0]["body"], "full-body-content");
    assert_eq!(history[0]["headers"]["content-type"], "application/json");
}
