use gatus_mcp_rs::client::GatusClient;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_gatus_client_trigger_check() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None);

    Mock::given(method("POST"))
        .and(path("/api/v1/endpoints/core_service-1/trigger"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let result = client.trigger_check("core_service-1").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_gatus_client_reload_config() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None);

    Mock::given(method("POST"))
        .and(path("/api/v1/config/reload"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let result = client.reload_config().await;
    assert!(result.is_ok());
}
