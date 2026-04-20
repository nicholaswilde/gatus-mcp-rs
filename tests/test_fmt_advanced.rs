use gatus_mcp_rs::client::{
    CorrelatedEvent, ExpiringCertificate, FailureSummary, FlappingService, GroupStats,
    PerformanceComparison,
};
use gatus_mcp_rs::fmt::{
    format_alert_correlation, format_expiring_certificates, format_failure_summary,
    format_flapping_services, format_group_stats, format_performance_comparison,
};

#[test]
fn test_format_expiring_certificates() {
    let certs = vec![ExpiringCertificate {
        name: "service-1".to_string(),
        group: "core".to_string(),
        expiration: 1000000,
        days_remaining: 1,
    }];
    let output = format_expiring_certificates(&certs);
    assert!(output.contains("| service-1 | core | 1 | 1000000 |"));
}

#[test]
fn test_format_failure_summary() {
    let summary = FailureSummary {
        name: "service-1".to_string(),
        group: "core".to_string(),
        failed_conditions: vec!["[STATUS] == 200".to_string()],
        passed_conditions: vec!["[RESPONSE_TIME] < 500".to_string()],
    };
    let output = format_failure_summary(&summary);
    assert!(output.contains("### Failure Summary for service-1 (core)"));
    assert!(output.contains("❌ Failed Conditions"));
    assert!(output.contains("- [STATUS] == 200"));
}

#[test]
fn test_format_group_stats() {
    let stats = GroupStats {
        group: "core".to_string(),
        total: 2,
        up: 1,
        down: 1,
        degraded: 0,
    };
    let output = format_group_stats(&stats);
    assert!(output.contains("### Group Health: core"));
    assert!(output.contains("- **Health Percentage:** 50.00%"));
}

#[test]
fn test_format_alert_correlation() {
    let events = vec![CorrelatedEvent {
        timestamp: "2026-04-19T00:00:00Z".to_string(),
        event_type: "alert".to_string(),
        description: "ALERT".to_string(),
    }];
    let output = format_alert_correlation(&events);
    assert!(output.contains("### Notification Correlation Timeline"));
    assert!(output.contains("| 2026-04-19T00:00:00Z | 🔔 alert | ALERT |"));
}

#[test]
fn test_format_flapping_services() {
    let services = vec![FlappingService {
        name: "service-1".to_string(),
        group: "core".to_string(),
        failure_count: 10,
        success_count: 100,
    }];
    let output = format_flapping_services(&services);
    assert!(output.contains("### Services with Failures (Flapping)"));
    assert!(output.contains("| service-1 | core | 10 | 100 |"));
}

#[test]
fn test_format_performance_comparison() {
    let comparison = PerformanceComparison {
        key: "core_service-1".to_string(),
        avg_1h: 100.0,
        avg_7d: 50.0,
        delta_percentage: 100.0,
    };
    let output = format_performance_comparison(&comparison);
    assert!(output.contains("### Performance Comparison for core_service-1"));
    assert!(output.contains("- **Delta:** +100.00%"));
    assert!(output.contains("Significant latency regression detected!"));
}
