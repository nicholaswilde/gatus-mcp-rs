use gatus_mcp_rs::client::GatusClient;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_gatus_client_get_expiring_certificates() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);

    // 29 days in nanoseconds (expiring soon)
    let expiring_soon = 29 * 24 * 60 * 60 * 1_000_000_000u64;
    // 31 days in nanoseconds (not expiring soon)
    let not_expiring_soon = 31 * 24 * 60 * 60 * 1_000_000_000u64;

    let gatus_response = json!([
        {
            "name": "expiring-service",
            "group": "core",
            "status": "UP",
            "results": [
                {
                    "success": true,
                    "timestamp": "2026-04-19T00:00:00Z",
                    "duration": 100,
                    "certificate_expiration": expiring_soon
                }
            ]
        },
        {
            "name": "healthy-service",
            "group": "core",
            "status": "UP",
            "results": [
                {
                    "success": true,
                    "timestamp": "2026-04-19T00:00:00Z",
                    "duration": 100,
                    "certificate_expiration": not_expiring_soon
                }
            ]
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/statuses"))
        .respond_with(ResponseTemplate::new(200).set_body_json(gatus_response))
        .mount(&mock_server)
        .await;

    let expiring = client.get_expiring_certificates(30).await.unwrap();
    assert_eq!(expiring.len(), 1);
    assert_eq!(expiring[0].name, "expiring-service");
}
