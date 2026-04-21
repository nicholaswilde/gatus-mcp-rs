use gatus_mcp_rs::client::PageHealth;
use gatus_mcp_rs::fmt::format_page_health;

#[test]
fn test_format_page_health() {
    let health = PageHealth {
        id: "page-1".to_string(),
        name: "Main Page".to_string(),
        up: 10,
        down: 2,
        degraded: 1,
    };

    let formatted = format_page_health(&health);
    println!("{}", formatted);
    assert!(formatted.contains("Status Page Health: Main Page (page-1)"));
    assert!(formatted.contains("**UP:** 10"));
    assert!(formatted.contains("**DOWN:** 2"));
    assert!(formatted.contains("**DEGRADED:** 1"));
    assert!(formatted.contains("76.92%")); // 10 / 13
}
