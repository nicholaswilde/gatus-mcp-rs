use gatus_mcp_rs::client::SuiteHealth;
use gatus_mcp_rs::fmt::format_suite_health;

#[test]
fn test_format_suite_health() {
    let health = SuiteHealth {
        id: "page-1".to_string(),
        name: "Main Suite".to_string(),
        up: 10,
        down: 2,
        degraded: 1,
    };

    let formatted = format_suite_health(&health);
    assert!(formatted.contains("Suite Health: Main Suite (page-1)"));
    assert!(formatted.contains("**UP:** 10"));
    assert!(formatted.contains("**DOWN:** 2"));
    assert!(formatted.contains("**DEGRADED:** 1"));
    assert!(formatted.contains("76.92%"));
}
