use gatus_mcp_rs::client::CertificateAudit;
use gatus_mcp_rs::fmt::format_certificate_audit;

#[test]
fn test_format_certificate_audit() {
    let audit = CertificateAudit {
        name: "service-1".to_string(),
        group: "core".to_string(),
        issuer: Some("Let's Encrypt".to_string()),
        algorithm: Some("RSA-2048".to_string()),
        sans: vec!["example.com".to_string(), "www.example.com".to_string()],
        expiration: Some(1000000000),
    };

    let formatted = format_certificate_audit(&audit);
    println!("{}", formatted);
    assert!(formatted.contains("Certificate Audit: service-1 (core)"));
    assert!(formatted.contains("Let's Encrypt"));
    assert!(formatted.contains("RSA-2048"));
    assert!(formatted.contains("example.com"));
    assert!(formatted.contains("www.example.com"));
    assert!(formatted.contains("days remaining"));
}
