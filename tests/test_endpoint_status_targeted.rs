use gatus_mcp_rs::client::GatusClient;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_endpoint_statuses_targeted() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), Some("test-key".to_string()));

    let mock_response = r#"[
        {
            "name": "service1",
            "group": "group1",
            "status": "UP",
            "results": [
                {
                    "timestamp": "2023-01-01T00:00:00Z",
                    "success": true,
                    "duration": 100,
                    "conditionResults": []
                }
            ],
            "events": []
        }
    ]"#;

    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/group1_service1/statuses"))
        .and(header("Authorization", "Bearer test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(mock_response, "application/json"))
        .mount(&mock_server)
        .await;

    let statuses = client
        .get_endpoint_statuses("group1_service1")
        .await
        .unwrap();
    assert_eq!(statuses.len(), 1);
    assert_eq!(statuses[0].name, "service1");
}
