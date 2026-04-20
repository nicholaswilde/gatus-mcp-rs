use gatus_mcp_rs::client::StatusPage;
use gatus_mcp_rs::fmt::format_status_pages;

#[test]
fn test_format_status_pages() {
    let pages = vec![
        StatusPage {
            id: "page-1".to_string(),
            name: "Main Page".to_string(),
        },
        StatusPage {
            id: "page-2".to_string(),
            name: "Internal Page".to_string(),
        },
    ];

    let output = format_status_pages(&pages);
    assert!(output.contains("### Gatus Status Pages"));
    assert!(output.contains("| page-1 | Main Page |"));
    assert!(output.contains("| page-2 | Internal Page |"));
}
