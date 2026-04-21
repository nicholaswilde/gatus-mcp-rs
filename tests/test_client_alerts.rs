use gatus_mcp_rs::client::GatusClient;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_gatus_client_get_alert_rules() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);

    let config_response = json!({
        "endpoints": [
            {
                "name": "service-1",
                "group": "core",
                "url": "http://localhost:8080",
                "conditions": ["[STATUS] == 200"],
                "alerts": [
                    {
                        "type": "slack",
                        "enabled": true,
                        "failure-threshold": 3,
                        "success-threshold": 2,
                        "description": "Service 1 is down"
                    }
                ]
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path("/api/v1/config"))
        .respond_with(ResponseTemplate::new(200).set_body_json(config_response))
        .mount(&mock_server)
        .await;

    let alert_rules = client.get_alert_rules().await.unwrap();
    assert_eq!(alert_rules.len(), 1);
    assert_eq!(alert_rules[0].endpoint, "service-1");
    assert_eq!(alert_rules[0].alert_type, "slack");
}

#[tokio::test]
async fn test_gatus_client_test_alert_notification() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);

    Mock::given(method("POST"))
        .and(path("/api/v1/endpoints/core_service-1/test-alert"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let result = client.test_alert_notification("core_service-1").await;
    assert!(result.is_ok());
}
