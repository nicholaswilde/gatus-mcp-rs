use gatus_mcp_rs::client::GatusClient;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_gatus_client_get_flapping_services() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);

    let metrics_response = "
gatus_results_total{group=\"core\",name=\"service-1\",success=\"true\",type=\"HTTP\"} 100
gatus_results_total{group=\"core\",name=\"service-1\",success=\"false\",type=\"HTTP\"} 50
gatus_results_total{group=\"core\",name=\"service-2\",success=\"true\",type=\"HTTP\"} 200
gatus_results_total{group=\"core\",name=\"service-2\",success=\"false\",type=\"HTTP\"} 0
";

    Mock::given(method("GET"))
        .and(path("/metrics"))
        .respond_with(ResponseTemplate::new(200).set_body_string(metrics_response))
        .mount(&mock_server)
        .await;

    let flapping = client.get_flapping_services().await.unwrap();
    assert_eq!(flapping.len(), 1);
    assert_eq!(flapping[0].name, "service-1");
    assert_eq!(flapping[0].failure_count, 50);
}
