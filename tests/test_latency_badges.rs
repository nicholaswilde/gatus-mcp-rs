use gatus_mcp_rs::client::GatusClient;

#[test]
fn test_get_latency_badge_url() {
    let client = GatusClient::new("https://status.example.org".to_string(), None, None, None);
    let url = client.get_latency_badge_url("core_service-1", "24h");
    assert_eq!(url, "https://status.example.org/api/v1/endpoints/core_service-1/response-times/24h/badge.svg");
}

#[test]
fn test_get_latency_chart_url() {
    let client = GatusClient::new("https://status.example.org".to_string(), None, None, None);
    let url = client.get_latency_chart_url("core_service-1", "24h");
    assert_eq!(url, "https://status.example.org/api/v1/endpoints/core_service-1/response-times/24h/chart.svg");
}
