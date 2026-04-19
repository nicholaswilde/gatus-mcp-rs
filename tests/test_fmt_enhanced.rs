use gatus_mcp_rs::client::{EndpointStatus, HealthResult, ConditionResult};
use gatus_mcp_rs::fmt::format_endpoint_status;

#[test]
fn test_format_endpoint_status_with_conditions() {
    let endpoint = EndpointStatus {
        name: "test-service".to_string(),
        group: "test-group".to_string(),
        status: Some("UP".to_string()),
        results: vec![
            HealthResult {
                timestamp: "2023-01-01T00:00:00Z".to_string(),
                success: true,
                hostname: None,
                ip: None,
                duration: 100_000_000,
                errors: vec![],
                status: Some(200),
                condition_results: vec![
                    ConditionResult {
                        condition: "[STATUS] == 200".to_string(),
                        success: true,
                    },
                    ConditionResult {
                        condition: "[RESPONSE_TIME] < 500".to_string(),
                        success: true,
                    }
                ],
                body: None,
                headers: None,
                certificate_expiration: None,
            }
        ],
        events: vec![],
    };

    let formatted = format_endpoint_status(&endpoint, None);
    
    // This should fail initially because conditions are not yet included in format_endpoint_status
    assert!(formatted.contains("#### Conditions"));
    assert!(formatted.contains("✅ [STATUS] == 200"));
    assert!(formatted.contains("✅ [RESPONSE_TIME] < 500"));
}
