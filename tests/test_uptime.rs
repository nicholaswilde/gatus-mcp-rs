use chrono::Utc;
use gatus_mcp_rs::client::{EndpointStatus, GatusClient, HealthResult};
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_gatus_client_get_uptime() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None);
    let now = Utc::now();
    let ts1 = (now - chrono::Duration::minutes(10)).to_rfc3339();
    let ts2 = (now - chrono::Duration::minutes(5)).to_rfc3339();

    let gatus_response = json!([
        {
            "name": "service-1",
            "group": "core",
            "status": "UP",
            "results": [
                {
                    "timestamp": ts1,
                    "success": true,
                    "duration": 100
                },
                {
                    "timestamp": ts2,
                    "success": false,
                    "duration": 100
                }
            ]
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/statuses"))
        .respond_with(ResponseTemplate::new(200).set_body_json(gatus_response))
        .mount(&mock_server)
        .await;

    let uptime = client.get_uptime("service-1", "24h").await.unwrap();
    assert_eq!(uptime, 50.0);
}

#[test]
fn test_endpoint_status_calculate_uptime() {
    let now = Utc::now();
    let ts1 = (now - chrono::Duration::minutes(10)).to_rfc3339();
    let ts2 = (now - chrono::Duration::minutes(5)).to_rfc3339();

    let endpoint = EndpointStatus {
        name: "test-service".to_string(),
        group: "test-group".to_string(),
        status: Some("UP".to_string()),
        results: vec![
            HealthResult {
                timestamp: ts1,
                success: true,
                hostname: None,
                ip: None,
                duration: 100,
                errors: vec![],
                status: Some(200),
                condition_results: vec![],
            },
            HealthResult {
                timestamp: ts2,
                success: false,
                hostname: None,
                ip: None,
                duration: 100,
                errors: vec![],
                status: Some(500),
                condition_results: vec![],
            },
        ],
        events: vec![],
    };

    // 1 success, 1 failure = 50.0% uptime
    assert_eq!(endpoint.calculate_uptime("24h"), 50.0);
}

#[test]
fn test_endpoint_status_calculate_uptime_no_results() {
    let endpoint = EndpointStatus {
        name: "test-service".to_string(),
        group: "test-group".to_string(),
        status: None,
        results: vec![],
        events: vec![],
    };

    assert_eq!(endpoint.calculate_uptime("24h"), 100.0);
}
