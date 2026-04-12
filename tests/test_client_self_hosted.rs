use gatus_mcp_rs::client::GatusClient;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_endpoint_uptimes() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None);

    let gatus_response = json!({
        "1672531200": 1.0,
        "1672534800": 0.98
    });

    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/Core_Frontend/uptimes/24h"))
        .respond_with(ResponseTemplate::new(200).set_body_json(gatus_response))
        .mount(&mock_server)
        .await;

    let result = client
        .get_endpoint_uptimes("Core_Frontend", "24h")
        .await
        .unwrap();

    assert_eq!(result.len(), 2);
    assert_eq!(result.get("1672531200"), Some(&1.0));
    assert_eq!(result.get("1672534800"), Some(&0.98));
}

#[tokio::test]
async fn test_get_endpoint_response_times() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None);

    let gatus_response = json!([
        {"timestamp": "2023-10-27T10:00:00Z", "value": 124},
        {"timestamp": "2023-10-27T10:01:00Z", "value": 135}
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/Core_Frontend/response-times/24h"))
        .respond_with(ResponseTemplate::new(200).set_body_json(gatus_response))
        .mount(&mock_server)
        .await;

    let result = client
        .get_endpoint_response_times("Core_Frontend", "24h")
        .await
        .unwrap();

    assert_eq!(result.len(), 2);
    assert_eq!(result[0].value, 124);
    assert_eq!(result[1].value, 135);
}

#[tokio::test]
async fn test_get_instance_health() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None);

    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(ResponseTemplate::new(200).set_body_string("UP"))
        .mount(&mock_server)
        .await;

    let result = client.get_instance_health().await.unwrap();

    assert_eq!(result, "UP");
}
