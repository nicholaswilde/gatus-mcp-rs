use gatus_mcp_rs::client::GatusClient;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_gatus_client_get_raw_results() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);

    let mock_response = json!([
        {
            "name": "service1",
            "group": "group1",
            "results": [
                {
                    "timestamp": "2023-01-01T00:00:00Z",
                    "success": true,
                    "duration": 100,
                    "body": "full-body-content",
                    "headers": {"X-Test": "Value"}
                }
            ]
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/group1_service1/statuses"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    // This should fail because get_raw_results is not yet implemented
    let results = client
        .get_raw_results("group1_service1", 10)
        .await
        .expect("Failed to get raw results");

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].body.as_deref(), Some("full-body-content"));
    assert_eq!(
        results[0]
            .headers
            .as_ref()
            .unwrap()
            .get("X-Test")
            .map(|v| v.as_str()),
        Some("Value")
    );
}
