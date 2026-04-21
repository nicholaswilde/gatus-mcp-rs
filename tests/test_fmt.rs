use gatus_mcp_rs::client::{ConditionResult, EndpointStatus, HealthResult, SystemStats};
use gatus_mcp_rs::fmt::{
    format_config_summary, format_endpoint_status, format_endpoints_summary, format_system_stats,
};

#[test]
fn test_format_endpoint_status() {
    let endpoint = EndpointStatus {
        name: "test-service".to_string(),
        group: "test-group".to_string(),
        status: Some("UP".to_string()),
        results: vec![HealthResult {
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            success: true,
            hostname: Some("host".to_string()),
            ip: Some("1.2.3.4".to_string()),
            duration: 100_000_000,
            errors: vec!["error1".to_string()],
            status: Some(200),
            condition_results: vec![],
            body: None,
            headers: None,
            certificate_expiration: None,
            certificate_issuer: None,
            certificate_algorithm: None,
            certificate_sans: None,
        }],
        events: vec![],
    };

    let output = format_endpoint_status(&endpoint, None);
    assert!(output.contains("### test-service"));
    assert!(output.contains("- **Group:** test-group"));
    assert!(output.contains("- **Status:** UP"));
    assert!(output.contains("#### Latest Result"));
    assert!(output.contains("- **Errors:**"));
    assert!(output.contains("  - error1"));
}

#[test]
fn test_format_endpoints_summary() {
    let endpoints = vec![
        EndpointStatus {
            name: "svc1".to_string(),
            group: "grp1".to_string(),
            status: Some("UP".to_string()),
            results: vec![HealthResult {
                timestamp: "ts".to_string(),
                success: true,
                hostname: None,
                ip: None,
                duration: 0,
                errors: vec![],
                status: None,
                condition_results: vec![],
                body: None,
                headers: None,
                certificate_expiration: None,
                certificate_issuer: None,
                certificate_algorithm: None,
                certificate_sans: None,
            }],
            events: vec![],
        },
        EndpointStatus {
            name: "svc2".to_string(),
            group: "grp2".to_string(),
            status: Some("DOWN".to_string()),
            results: vec![HealthResult {
                timestamp: "ts".to_string(),
                success: false,
                hostname: None,
                ip: None,
                duration: 0,
                errors: vec![],
                status: None,
                condition_results: vec![],
                body: None,
                headers: None,
                certificate_expiration: None,
                certificate_issuer: None,
                certificate_algorithm: None,
                certificate_sans: None,
            }],
            events: vec![],
        },
        EndpointStatus {
            name: "svc3".to_string(),
            group: "grp3".to_string(),
            status: None,
            results: vec![],
            events: vec![],
        },
    ];

    let output = format_endpoints_summary(&endpoints);
    assert!(output.contains("| svc1 | grp1 | UP | ✅ |"));
    assert!(output.contains("| svc2 | grp2 | DOWN | ❌ |"));
    assert!(output.contains("| svc3 | grp3 | UNKNOWN | ❓ |"));
}

#[test]
fn test_format_system_stats() {
    let stats = SystemStats {
        total: 10,
        up: 8,
        down: 1,
        degraded: 1,
        certificates_expiring_soon: 0,
    };

    let output = format_system_stats(&stats);
    assert!(output.contains("- **Total Endpoints:** 10"));
    assert!(output.contains("- **UP:** 8"));
    assert!(output.contains("- **DOWN:** 1"));
    assert!(output.contains("- **DEGRADED:** 1"));
    assert!(output.contains("- **Certificates Expiring Soon:** 0"));
}

#[test]
fn test_format_config_summary() {
    let endpoints = vec![EndpointStatus {
        name: "svc1".to_string(),
        group: "grp1".to_string(),
        status: None,
        results: vec![HealthResult {
            timestamp: "ts".to_string(),
            success: true,
            hostname: None,
            ip: None,
            duration: 0,
            errors: vec![],
            status: None,
            condition_results: vec![
                ConditionResult {
                    condition: "[STATUS] == 200".to_string(),
                    success: true,
                },
                ConditionResult {
                    condition: "[BODY] == ok".to_string(),
                    success: false,
                },
            ],
            body: None,
            headers: None,
            certificate_expiration: None,
            certificate_issuer: None,
            certificate_algorithm: None,
            certificate_sans: None,
        }],
        events: vec![],
    }];

    let output = format_config_summary(&endpoints);
    assert!(output.contains("#### svc1 (Group: grp1)"));
    assert!(output.contains("- **Conditions:**"));
    assert!(output.contains("  - [STATUS] == 200 ✅"));
    assert!(output.contains("  - [BODY] == ok ❌"));
}
