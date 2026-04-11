use gatus_mcp_rs::client::GatusClient;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_gatus_client_get_alert_history() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None);

    let gatus_response = json!([
        {
            "name": "service-1",
            "group": "core",
            "status": "UP",
            "results": [],
            "events": [
                {
                    "type": "HEALTHY",
                    "timestamp": "2024-04-11T08:45:00Z"
                },
                {
                    "type": "UNHEALTHY",
                    "timestamp": "2024-04-11T08:40:00Z"
                }
            ]
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/statuses"))
        .respond_with(ResponseTemplate::new(200).set_body_json(gatus_response))
        .mount(&mock_server)
        .await;

    let history = client.get_alert_history(5).await.unwrap();
    assert_eq!(history.len(), 2);
    assert_eq!(history[0].service, "service-1");
    assert_eq!(history[0].event_type, "HEALTHY");
    assert_eq!(history[1].event_type, "UNHEALTHY");
}
