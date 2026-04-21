use gatus_mcp_rs::client::AlertRule;
use gatus_mcp_rs::fmt::format_alert_rules;

#[test]
fn test_format_alert_rules() {
    let rules = vec![
        AlertRule {
            endpoint: "service-1".to_string(),
            group: "core".to_string(),
            alert_type: "slack".to_string(),
            enabled: true,
            failure_threshold: 3,
            success_threshold: 2,
            description: Some("Service 1 down".to_string()),
        },
        AlertRule {
            endpoint: "service-2".to_string(),
            group: "core".to_string(),
            alert_type: "email".to_string(),
            enabled: false,
            failure_threshold: 1,
            success_threshold: 1,
            description: None,
        },
    ];

    let formatted = format_alert_rules(&rules);
    assert!(formatted.contains("service-1"));
    assert!(formatted.contains("slack"));
    assert!(formatted.contains("✅")); // Enabled icon
    assert!(formatted.contains("❌")); // Disabled icon
    assert!(formatted.contains("3 / 2")); // Thresholds
}
