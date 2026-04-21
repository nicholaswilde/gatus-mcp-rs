use gatus_mcp_rs::client::{GatusClient, HealthResult};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_gatus_client_push_endpoint_result() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), Some("test-key".to_string()), None, None);

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
        certificate_issuer: None,
        certificate_algorithm: None,
        certificate_sans: None,
    };

    Mock::given(method("POST"))
        .and(path("/api/v1/endpoints/test-endpoint/external"))
        .and(header("Authorization", "Bearer test-key"))
        .and(wiremock::matchers::query_param("success", "true"))
        .and(wiremock::matchers::query_param("duration", "100"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let push_result = client.push_endpoint_result("test-endpoint", result).await;
    assert!(push_result.is_ok());
}

#[tokio::test]
async fn test_gatus_client_push_endpoint_result_error() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);

    let result = HealthResult {
        timestamp: "2023-01-01T00:00:00Z".to_string(),
        success: false,
        hostname: None,
        ip: None,
        duration: 500,
        errors: vec!["Timeout".to_string()],
        status: None,
        condition_results: vec![],
        body: None,
        headers: None,
        certificate_expiration: None,
        certificate_issuer: None,
        certificate_algorithm: None,
        certificate_sans: None,
    };

    Mock::given(method("POST"))
        .and(path("/api/v1/endpoints/test-endpoint/external"))
        .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
        .mount(&mock_server)
        .await;

    let push_result = client.push_endpoint_result("test-endpoint", result).await;
    assert!(push_result.is_err());
    assert!(push_result.unwrap_err().to_string().contains("status 500"));
}
