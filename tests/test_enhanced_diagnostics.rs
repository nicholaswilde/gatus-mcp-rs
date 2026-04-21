use gatus_mcp_rs::client::{EndpointStatus, HealthResult, SystemStats};
use serde_json::json;
use std::collections::HashMap;

#[test]
fn test_system_stats_with_ssl() {
    let stats = SystemStats {
        total: 2,
        up: 2,
        down: 0,
        degraded: 0,
        certificates_expiring_soon: 1,
    };
    assert_eq!(stats.certificates_expiring_soon, 1);
}

#[test]
fn test_deserialize_health_result_with_diagnostics() {
    let data = json!({
        "timestamp": "2026-04-17T12:00:00Z",
        "success": false,
        "hostname": "example.com",
        "ip": "1.2.3.4",
        "duration": 150000000,
        "errors": ["connection timed out"],
        "status": 500,
        "conditionResults": [
            {
                "condition": "[STATUS] == 200",
                "success": false
            }
        ],
        "body": "Error message from server",
        "headers": {
            "Content-Type": "text/plain",
            "Server": "nginx"
        },
        "certificate_expiration": 7776000000000000u64
    });

    let result: HealthResult =
        serde_json::from_value(data).expect("Failed to deserialize HealthResult");

    assert_eq!(result.body.as_deref(), Some("Error message from server"));

    let headers = result.headers.as_ref().expect("Headers should be present");
    assert_eq!(
        headers.get("Content-Type").map(|s| s.as_str()),
        Some("text/plain")
    );
    assert_eq!(headers.get("Server").map(|s| s.as_str()), Some("nginx"));

    assert_eq!(result.certificate_expiration, Some(7776000000000000));
}

#[test]
fn test_deserialize_health_result_without_diagnostics() {
    let data = json!({
        "timestamp": "2026-04-17T12:00:00Z",
        "success": true,
        "hostname": "example.com",
        "ip": "1.2.3.4",
        "duration": 50000000,
        "errors": [],
        "status": 200,
        "conditionResults": [
            {
                "condition": "[STATUS] == 200",
                "success": true
            }
        ]
    });

    let result: HealthResult =
        serde_json::from_value(data).expect("Failed to deserialize HealthResult");

    assert!(result.body.is_none());
    assert!(result.headers.is_none());
    assert!(result.certificate_expiration.is_none());
}

#[test]
fn test_format_system_stats_with_ssl() {
    use gatus_mcp_rs::fmt::format_system_stats;
    let stats = SystemStats {
        total: 10,
        up: 8,
        down: 1,
        degraded: 1,
        certificates_expiring_soon: 3,
    };
    let formatted = format_system_stats(&stats);
    assert!(formatted.contains("- **Certificates Expiring Soon:** 3"));
}

#[test]
fn test_format_endpoint_status_with_diagnostics() {
    use gatus_mcp_rs::fmt::format_endpoint_status;
    let endpoint = EndpointStatus {
        name: "test-service".to_string(),
        group: "test-group".to_string(),
        status: Some("UP".to_string()),
        results: vec![HealthResult {
            timestamp: "2026-04-17T12:00:00Z".to_string(),
            success: false,
            hostname: Some("example.com".to_string()),
            ip: Some("1.2.3.4".to_string()),
            duration: 150000000,
            errors: vec!["connection timeout".to_string()],
            status: Some(500),
            condition_results: vec![],
            body: Some("Error body".to_string()),
            headers: Some({
                let mut h = HashMap::new();
                h.insert("X-Error".to_string(), "True".to_string());
                h
            }),
            certificate_expiration: Some(7776000000000000u64), // 90 days
            certificate_issuer: None,
            certificate_algorithm: None,
            certificate_sans: None,
        }],
        events: vec![],
    };

    let formatted = format_endpoint_status(&endpoint, None);
    assert!(formatted.contains("- **SSL Expiration:** 90 days remaining"));
    assert!(formatted.contains("- **Headers:** (present)"));
    assert!(formatted.contains("- **Body Snippet:** Error body"));
}
