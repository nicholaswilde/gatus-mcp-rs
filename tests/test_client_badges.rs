use gatus_mcp_rs::client::GatusClient;

#[test]
fn test_get_badge_url() {
    let client = GatusClient::new("https://status.example.org".to_string(), None);
    let url = client.get_badge_url("core_ext-ep-test");
    assert_eq!(url, "https://status.example.org/api/v1/endpoints/core_ext-ep-test/health/badge.svg");
}

#[test]
fn test_get_uptime_badge_url() {
    let client = GatusClient::new("https://status.example.org".to_string(), None);
    let url = client.get_uptime_badge_url("core_ext-ep-test", "24h");
    assert_eq!(url, "https://status.example.org/api/v1/endpoints/core_ext-ep-test/uptimes/24h/badge.svg");
}
