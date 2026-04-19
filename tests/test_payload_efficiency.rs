use gatus_mcp_rs::client::GatusClient;
use gatus_mcp_rs::mcp::McpHandler;
use serde_json::{json, Value};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_service_history_payload_efficiency() {
    let mock_server = MockServer::start().await;

    let history_with_payloads = json!([
        {
            "name": "test-service",
            "group": "test-group",
            "results": [
                {
                    "timestamp": "2026-04-17T12:00:00Z",
                    "success": true,
                    "duration": 50000000,
                    "status": 200,
                    "conditionResults": [],
                    "body": "Large body that should be stripped on success",
                    "headers": {
                        "Content-Type": "application/json"
                    }
                },
                {
                    "timestamp": "2026-04-17T11:00:00Z",
                    "success": false,
                    "duration": 150000000,
                    "status": 500,
                    "conditionResults": [],
                    "body": "Error body that should be KEPT on failure",
                    "headers": {
                        "Content-Type": "text/plain"
                    }
                }
            ]
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/test-service/statuses"))
        .respond_with(ResponseTemplate::new(200).set_body_json(history_with_payloads))
        .mount(&mock_server)
        .await;

    let client = GatusClient::new(mock_server.uri(), None, None, None);
    let handler = McpHandler::new(client);

    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "get_metrics",
            "arguments": {
                "action": "service-history",
                "id": "test-service"
            }
        },
        "id": 1
    });

    let response = handler.handle(request).await;

    let result_text = response["result"]["content"][0]["text"]
        .as_str()
        .expect("Response text not found");
    let history: Value = serde_json::from_str(result_text).expect("Failed to parse history JSON");

    assert!(history.is_array());
    let history_arr = history.as_array().unwrap();
    assert_eq!(history_arr.len(), 2);

    // First entry is success: true
    assert_eq!(history_arr[0]["success"], true);
    // CURRENT BEHAVIOR (Should fail if we want it to be thin):
    // assert!(history_arr[0]["body"].is_null());
    // assert!(history_arr[0]["headers"].is_null());

    // Desired behavior:
    assert!(
        history_arr[0]["body"].is_null(),
        "Success body should be stripped"
    );
    assert!(
        history_arr[0]["headers"].is_null(),
        "Success headers should be stripped"
    );

    // Second entry is success: false
    assert_eq!(history_arr[1]["success"], false);
    assert_eq!(
        history_arr[1]["body"],
        "Error body that should be KEPT on failure"
    );
    assert!(history_arr[1]["headers"].is_null());
}
