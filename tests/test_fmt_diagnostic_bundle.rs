use gatus_mcp_rs::client::{AlertEvent, DiagnosticBundle, FailureSummary, HealthResult};
use gatus_mcp_rs::fmt::format_diagnostic_bundle;

#[test]
fn test_format_diagnostic_bundle() {
    let bundle = DiagnosticBundle {
        name: "service-1".to_string(),
        group: "core".to_string(),
        results: vec![HealthResult {
            timestamp: "2023-01-01T00:00:00Z".to_string(),
            success: false,
            hostname: Some("localhost".to_string()),
            ip: Some("127.0.0.1".to_string()),
            duration: 100000000,
            errors: vec!["connection refused".to_string()],
            status: Some(500),
            condition_results: vec![],
            body: None,
            headers: None,
            certificate_expiration: None,
            certificate_issuer: None,
            certificate_algorithm: None,
            certificate_sans: None,
        }],
        failure_summary: FailureSummary {
            name: "service-1".to_string(),
            group: "core".to_string(),
            failed_conditions: vec!["[STATUS] == 200".to_string()],
            passed_conditions: vec![],
        },
        alert_events: vec![AlertEvent {
            service: "service-1".to_string(),
            group: "core".to_string(),
            event_type: "alert".to_string(),
            timestamp: "2023-01-01T00:00:00Z".to_string(),
        }],
    };

    let formatted = format_diagnostic_bundle(&bundle);
    println!("{}", formatted);
    assert!(formatted.contains("Diagnostic Bundle: service-1 (core)"));
    assert!(formatted.contains("connection refused"));
    assert!(formatted.contains("[STATUS] == 200"));
    assert!(formatted.contains("alert"));
}
