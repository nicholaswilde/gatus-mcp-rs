use gatus_mcp_rs::client::GatusClient;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_gatus_client_compare_performance() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);

    // Mock 1h response times: avg 100ms
    let response_1h = json!([
        {"timestamp": "2026-04-19T00:00:00Z", "value": 100},
        {"timestamp": "2026-04-19T00:05:00Z", "value": 100}
    ]);

    // Mock 7d response times: avg 50ms
    let response_7d = json!([
        {"timestamp": "2026-04-12T00:00:00Z", "value": 50},
        {"timestamp": "2026-04-13T00:00:00Z", "value": 50}
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/core_service-1/response-times/1h"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_1h))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/core_service-1/response-times/7d"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_7d))
        .mount(&mock_server)
        .await;

    let comparison = client.compare_performance("core_service-1").await.unwrap();
    assert_eq!(comparison.avg_1h, 100.0);
    assert_eq!(comparison.avg_7d, 50.0);
    assert_eq!(comparison.delta_percentage, 100.0);
}
