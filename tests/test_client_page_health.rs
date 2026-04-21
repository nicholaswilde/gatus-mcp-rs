use gatus_mcp_rs::client::GatusClient;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_gatus_client_get_page_health() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);

    let gatus_response = json!({
        "id": "page-1",
        "name": "Main Status Page",
        "endpoints": [
            {
                "name": "service-1",
                "group": "core",
                "status": "UP"
            },
            {
                "name": "service-2",
                "group": "core",
                "status": "DOWN"
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path("/api/v1/external/status-pages/page-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(gatus_response))
        .mount(&mock_server)
        .await;

    let health = client.get_page_health("page-1").await.unwrap();
    assert_eq!(health.id, "page-1");
    assert_eq!(health.up, 1);
    assert_eq!(health.down, 1);
}
