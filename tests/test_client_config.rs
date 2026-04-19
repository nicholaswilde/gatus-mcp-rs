use gatus_mcp_rs::client::GatusClient;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_gatus_client_get_config() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);

    let config_response = json!({
        "endpoints": [
            {
                "name": "service-1",
                "group": "core",
                "url": "http://localhost:8080"
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path("/api/v1/config"))
        .respond_with(ResponseTemplate::new(200).set_body_json(config_response))
        .mount(&mock_server)
        .await;

    let config = client.get_config().await.unwrap();
    assert_eq!(config["endpoints"].as_array().unwrap().len(), 1);
}
