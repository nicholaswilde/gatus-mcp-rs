use gatus_mcp_rs::client::GatusClient;
use gatus_mcp_rs::mcp::McpHandler;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_mcp_service_history_targeted() {
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
                    "body": "success-body",
                    "headers": {"content-type": "application/json"}
                },
                {
                    "timestamp": "2023-01-01T00:01:00Z",
                    "success": false,
                    "duration": 200,
                    "conditionResults": [],
                    "body": "failure-body-very-long-should-be-truncated-at-one-hundred-characters-long-so-we-can-test-the-truncation-logic-correctly",
                    "headers": {"content-type": "text/plain"}
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
                "action": "service-history",
                "id": "group1_service1",
                "limit": 2
            }
        },
        "id": 1
    });

    let response = handler.handle(request).await;
    let result_text = response["result"]["content"][0]["text"].as_str().unwrap();
    let history: serde_json::Value = serde_json::from_str(result_text).unwrap();

    assert_eq!(history.as_array().unwrap().len(), 2);

    // First result: success, body and headers should be stripped
    assert!(history[0]["body"].is_null());
    assert!(history[0]["headers"].is_null());
    assert_eq!(history[0]["success"], true);

    // Second result: failure, body should be truncated, headers should be stripped
    assert!(history[1]["body"].as_str().unwrap().ends_with("..."));
    assert!(history[1]["body"].as_str().unwrap().len() <= 103); // 100 + "..."
    assert!(history[1]["headers"].is_null());
    assert_eq!(history[1]["success"], false);
}
