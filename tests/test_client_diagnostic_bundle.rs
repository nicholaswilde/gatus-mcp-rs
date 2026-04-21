use gatus_mcp_rs::client::GatusClient;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_gatus_client_get_diagnostic_bundle() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);

    let health_response = json!([
        {
            "name": "service-1",
            "group": "core",
            "results": [
                {
                    "timestamp": "2023-01-01T00:00:00Z",
                    "success": false,
                    "errors": ["connection refused"],
                    "duration": 100,
                    "conditionResults": [
                        {
                            "condition": "[STATUS] == 200",
                            "success": false
                        }
                    ]
                }
            ]
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/core_service-1/statuses"))
        .respond_with(ResponseTemplate::new(200).set_body_json(health_response))
        .mount(&mock_server)
        .await;

    let bundle = client
        .get_diagnostic_bundle("core_service-1")
        .await
        .unwrap();
    assert_eq!(bundle.name, "service-1");
    assert!(!bundle.results.is_empty());
    assert!(!bundle.results[0].success);
}
