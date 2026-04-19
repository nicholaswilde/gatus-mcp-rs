use gatus_mcp_rs::client::GatusClient;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_gatus_client_basic_auth() {
    let mock_server = MockServer::start().await;

    let client = GatusClient::new(
        mock_server.uri(),
        None,
        Some("admin".to_string()),
        Some("password123".to_string()),
    );

    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/statuses"))
        // Basic admin:password123 in base64 is YWRtaW46cGFzc3dvcmQxMjM=
        .and(header("Authorization", "Basic YWRtaW46cGFzc3dvcmQxMjM="))
        .respond_with(ResponseTemplate::new(200).set_body_json(vec![] as Vec<serde_json::Value>))
        .mount(&mock_server)
        .await;

    let result = client.list_services(false).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_gatus_client_both_auth_prefers_api_key() {
    let mock_server = MockServer::start().await;

    let client = GatusClient::new(
        mock_server.uri(),
        Some("api-token".to_string()),
        Some("admin".to_string()),
        Some("password123".to_string()),
    );

    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/statuses"))
        .and(header("Authorization", "Bearer api-token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(vec![] as Vec<serde_json::Value>))
        .mount(&mock_server)
        .await;

    let result = client.list_services(false).await;
    assert!(result.is_ok());
}

#[test]
fn test_gatus_client_sanitize_key() {
    let client = GatusClient::new("http://localhost".to_string(), None, None, None);

    // Spaces and & should be replaced by hyphens
    assert_eq!(
        client.sanitize_key("Authentication & Security"),
        "Authentication---Security"
    );

    // Normal name should be unchanged
    assert_eq!(client.sanitize_key("Authentik"), "Authentik");

    // Combined key
    let group = "Authentication & Security";
    let name = "Authentik";
    let key = format!(
        "{}_{}",
        client.sanitize_key(group),
        client.sanitize_key(name)
    );
    assert_eq!(key, "Authentication---Security_Authentik");
}
