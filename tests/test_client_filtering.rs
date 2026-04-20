use gatus_mcp_rs::client::GatusClient;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_gatus_client_list_services_with_filter() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);

    let gatus_response = json!([
        {
            "name": "service-up",
            "group": "core",
            "status": "UP",
            "results": [{"success": true, "timestamp": "2026-04-19T00:00:00Z", "duration": 100}]
        },
        {
            "name": "service-down",
            "group": "core",
            "status": "DOWN",
            "results": [{"success": false, "timestamp": "2026-04-19T00:00:00Z", "duration": 100}]
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/statuses"))
        .respond_with(ResponseTemplate::new(200).set_body_json(gatus_response))
        .mount(&mock_server)
        .await;

    // Filter by DOWN status
    let services = client.list_services(false, Some("DOWN")).await.unwrap();
    assert_eq!(services.len(), 1);
    assert_eq!(services[0].name, "service-down");
}
