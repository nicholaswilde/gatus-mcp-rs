use gatus_mcp_rs::client::{GatusClient, HealthResult};
use serde_json::json;
use wiremock::matchers::{body_json, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_gatus_client_push_endpoint_result() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), Some("test-key".to_string()));

    let result = HealthResult {
        timestamp: "2023-01-01T00:00:00Z".to_string(),
        success: true,
        hostname: Some("test-host".to_string()),
        ip: Some("127.0.0.1".to_string()),
        duration: 100,
        errors: vec![],
        status: Some(200),
        condition_results: vec![],
        body: Some("OK".to_string()),
        headers: None,
        certificate_expiration: None,
    };

    Mock::given(method("POST"))
        .and(path("/api/v1/endpoints/test-endpoint/results"))
        .and(header("Authorization", "Bearer test-key"))
        .and(body_json(json!({
            "timestamp": "2023-01-01T00:00:00Z",
            "success": true,
            "hostname": "test-host",
            "ip": "127.0.0.1",
            "duration": 100,
            "errors": [],
            "status": 200,
            "conditionResults": [],
            "body": "OK",
            "headers": null,
            "certificate_expiration": null
        })))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let push_result = client.push_endpoint_result("test-endpoint", result).await;
    assert!(push_result.is_ok());
}
