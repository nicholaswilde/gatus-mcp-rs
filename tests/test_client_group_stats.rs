use gatus_mcp_rs::client::GatusClient;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_gatus_client_get_group_stats() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);

    let gatus_response = json!([
        {
            "name": "service-1",
            "group": "core",
            "status": "UP",
            "results": [{"success": true, "timestamp": "2026-04-19T00:00:00Z", "duration": 100}]
        },
        {
            "name": "service-2",
            "group": "core",
            "status": "DOWN",
            "results": [{"success": false, "timestamp": "2026-04-19T00:00:00Z", "duration": 100}]
        },
        {
            "name": "service-3",
            "group": "other",
            "status": "UP",
            "results": [{"success": true, "timestamp": "2026-04-19T00:00:00Z", "duration": 100}]
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/statuses"))
        .respond_with(ResponseTemplate::new(200).set_body_json(gatus_response))
        .mount(&mock_server)
        .await;

    let stats = client.get_group_stats("core").await.unwrap();
    assert_eq!(stats.group, "core");
    assert_eq!(stats.total, 2);
    assert_eq!(stats.up, 1);
    assert_eq!(stats.down, 1);
    assert_eq!(stats.degraded, 0);
}
