use gatus_mcp_rs::client::Suite;
use gatus_mcp_rs::fmt::format_suites;

#[test]
fn test_format_suites() {
    let pages = vec![
        Suite {
            id: "page-1".to_string(),
            name: "Main Page".to_string(),
        },
        Suite {
            id: "page-2".to_string(),
            name: "Internal Page".to_string(),
        },
    ];

    let output = format_suites(&pages);
    assert!(output.contains("### Gatus Suites"));
    assert!(output.contains("| page-1 | Main Page |"));
    assert!(output.contains("| page-2 | Internal Page |"));
}
