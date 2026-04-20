use gatus_mcp_rs::client::GatusClient;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_gatus_client_get_notification_events() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);

    let gatus_response = json!([
        {
            "name": "service-1",
            "group": "core",
            "status": "UP",
            "results": [
                {
                    "success": true,
                    "timestamp": "2026-04-19T00:10:00Z",
                    "duration": 100
                },
                {
                    "success": false,
                    "timestamp": "2026-04-19T00:05:00Z",
                    "duration": 100,
                    "errors": ["Connection timeout"]
                }
            ],
            "events": [
                {
                    "type": "RESOLVED",
                    "timestamp": "2026-04-19T00:11:00Z"
                },
                {
                    "type": "ALERT",
                    "timestamp": "2026-04-19T00:06:00Z"
                }
            ]
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/core_service-1/statuses"))
        .respond_with(ResponseTemplate::new(200).set_body_json(gatus_response))
        .mount(&mock_server)
        .await;

    let events = client.get_notification_events("core_service-1").await.unwrap();
    assert_eq!(events.len(), 4);
    // Newest first
    assert_eq!(events[0].event_type, "alert");
    assert_eq!(events[0].description, "RESOLVED");
    assert_eq!(events[1].event_type, "result");
    assert_eq!(events[1].description, "Success");
}
