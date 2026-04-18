use gatus_mcp_rs::client::{EndpointStatus, HealthResult, SystemStats};
use gatus_mcp_rs::fmt::{format_endpoint_status, format_system_stats};
use std::collections::HashMap;

#[test]
fn test_format_system_stats_with_ssl() {
    let stats = SystemStats {
        total: 10,
        up: 8,
        down: 1,
        degraded: 1,
        certificates_expiring_soon: 2,
    };
    let output = format_system_stats(&stats);
    assert!(output.contains("**Certificates Expiring Soon:** 2"));
}

#[test]
fn test_format_endpoint_status_with_diagnostics() {
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());

    let result = HealthResult {
        timestamp: "2026-04-17T12:00:00Z".to_string(),
        success: false,
        hostname: Some("example.com".to_string()),
        ip: Some("1.2.3.4".to_string()),
        duration: 150000000,
        errors: vec!["some error".to_string()],
        status: Some(500),
        condition_results: vec![],
        body: Some("{\"error\": \"internal server error\"}".to_string()),
        headers: Some(headers),
        certificate_expiration: Some(7776000000000000u64),
    };

    let endpoint = EndpointStatus {
        name: "test-service".to_string(),
        group: "test-group".to_string(),
        status: Some("DOWN".to_string()),
        results: vec![result],
        events: vec![],
    };

    let output = format_endpoint_status(&endpoint);
    assert!(output.contains("**SSL Expiration:** 90 days remaining"));
    assert!(output.contains("**Headers:** (present)"));
    assert!(output.contains("**Body Snippet:** {\"error\": \"internal server error\"}"));
}
