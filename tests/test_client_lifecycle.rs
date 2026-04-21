use gatus_mcp_rs::client::{EndpointConfig, GatusClient};
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_gatus_client_list_suites() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);

    let list_response = json!([
        { "key": "page-1", "name": "Main Page" }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v1/suites/statuses"))
        .respond_with(ResponseTemplate::new(200).set_body_json(list_response))
        .mount(&mock_server)
        .await;

    let pages = client.list_suites().await.unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].id, "page-1");
}

#[tokio::test]
async fn test_gatus_client_create_endpoint() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);

    let config = EndpointConfig {
        name: "endpoint-1".to_string(),
        group: Some("group-1".to_string()),
        url: "http://localhost:8080".to_string(),
        interval: Some("1m".to_string()),
        conditions: vec!["[STATUS] == 200".to_string()],
        method: None,
        body: None,
        headers: None,
        alerts: vec![],
    };

    Mock::given(method("POST"))
        .and(path("/api/v1/external/status-pages/page-1/endpoints"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let result = client.create_endpoint("page-1", config).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_gatus_client_update_endpoint() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);

    let config = EndpointConfig {
        name: "endpoint-1".to_string(),
        group: Some("group-1".to_string()),
        url: "http://localhost:8080".to_string(),
        interval: Some("1m".to_string()),
        conditions: vec!["[STATUS] == 200".to_string()],
        method: None,
        body: None,
        headers: None,
        alerts: vec![],
    };

    Mock::given(method("PUT"))
        .and(path(
            "/api/v1/external/status-pages/page-1/endpoints/endpoint-1",
        ))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let result = client.update_endpoint("page-1", "endpoint-1", config).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_gatus_client_delete_endpoint() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);

    Mock::given(method("DELETE"))
        .and(path(
            "/api/v1/external/status-pages/page-1/endpoints/endpoint-1",
        ))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let result = client.delete_endpoint("page-1", "endpoint-1").await;
    assert!(result.is_ok());
}
