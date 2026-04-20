use gatus_mcp_rs::client::GatusClient;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_gatus_client_get_failure_summary() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);

    let gatus_response = json!([
        {
            "name": "service-1",
            "group": "core",
            "status": "DOWN",
            "results": [
                {
                    "success": false,
                    "timestamp": "2026-04-19T00:00:00Z",
                    "duration": 100,
                    "conditionResults": [
                        {"condition": "[STATUS] == 200", "success": true},
                        {"condition": "[RESPONSE_TIME] < 50", "success": false}
                    ]
                }
            ]
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/core_service-1/statuses"))
        .respond_with(ResponseTemplate::new(200).set_body_json(gatus_response))
        .mount(&mock_server)
        .await;

    let summary = client.get_failure_summary("core_service-1").await.unwrap();
    assert_eq!(summary.name, "service-1");
    assert_eq!(summary.failed_conditions.len(), 1);
    assert_eq!(summary.failed_conditions[0], "[RESPONSE_TIME] < 50");
    assert_eq!(summary.passed_conditions.len(), 1);
    assert_eq!(summary.passed_conditions[0], "[STATUS] == 200");
}
