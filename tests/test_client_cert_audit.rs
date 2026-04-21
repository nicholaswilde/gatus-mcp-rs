use gatus_mcp_rs::client::GatusClient;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_gatus_client_get_certificate_audit() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);

    let health_response = json!([
        {
            "name": "service-1",
            "group": "core",
            "results": [
                {
                    "timestamp": "2023-01-01T00:00:00Z",
                    "success": true,
                    "duration": 100000000,
                    "conditionResults": [],
                    "certificateExpiration": 1000000000,
                    "certificateIssuer": "Let's Encrypt",
                    "certificateAlgorithm": "RSA-2048",
                    "certificateSans": ["example.com", "www.example.com"]
                }
            ]
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/core_service-1/statuses"))
        .respond_with(ResponseTemplate::new(200).set_body_json(health_response))
        .mount(&mock_server)
        .await;

    let audit = client
        .get_certificate_audit("core_service-1")
        .await
        .unwrap();
    assert_eq!(audit.name, "service-1");
    assert_eq!(audit.issuer, Some("Let's Encrypt".to_string()));
    assert_eq!(audit.algorithm, Some("RSA-2048".to_string()));
    assert_eq!(
        audit.sans,
        vec!["example.com".to_string(), "www.example.com".to_string()]
    );
}
