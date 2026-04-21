use gatus_mcp_rs::client::GatusClient;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_gatus_client_get_suite_health() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);

    let gatus_response = json!({
        "key": "page-1",
        "name": "Main Suite",
        "results": [
            {
                "endpointResults": [
                    { "success": true },
                    { "success": false }
                ]
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path("/api/v1/suites/page-1/statuses"))
        .respond_with(ResponseTemplate::new(200).set_body_json(gatus_response))
        .mount(&mock_server)
        .await;

    let health = client.get_suite_health("page-1").await.unwrap();
    assert_eq!(health.id, "page-1");
    assert_eq!(health.up, 1);
    assert_eq!(health.down, 1);
}
