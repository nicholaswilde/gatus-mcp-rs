use gatus_mcp_rs::client::{EndpointStatus, GatusClient};

#[test]
fn test_endpoint_status_get_key() {
    let endpoint = EndpointStatus {
        name: "Authentik".to_string(),
        group: "Authentication & Security".to_string(),
        status: None,
        results: vec![],
        events: vec![],
    };

    assert_eq!(endpoint.get_key(), "Authentication---Security_Authentik");
}

#[test]
fn test_endpoint_status_get_key_no_special_chars() {
    let endpoint = EndpointStatus {
        name: "service1".to_string(),
        group: "group1".to_string(),
        status: None,
        results: vec![],
        events: vec![],
    };

    assert_eq!(endpoint.get_key(), "group1_service1");
}

#[test]
fn test_client_sanitize_key() {
    let client = GatusClient::new("http://localhost".to_string(), None, None, None);
    assert_eq!(client.sanitize_key("My Service!"), "My-Service-");
}
