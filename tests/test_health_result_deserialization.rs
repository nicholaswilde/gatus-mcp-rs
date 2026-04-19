use gatus_mcp_rs::client::HealthResult;
use serde_json::json;

#[test]
fn test_health_result_deserialization_full() {
    let raw_json = json!({
        "timestamp": "2023-01-01T00:00:00Z",
        "success": true,
        "hostname": "test-host",
        "ip": "1.2.3.4",
        "duration": 12345678,
        "errors": ["error1"],
        "status": 200,
        "conditionResults": [
            {
                "condition": "[STATUS] == 200",
                "success": true
            }
        ],
        "body": "test-body",
        "headers": {
            "Content-Type": "text/plain"
        },
        "certificate_expiration": 3600
    });

    let result: HealthResult = serde_json::from_value(raw_json).unwrap();

    assert_eq!(result.timestamp, "2023-01-01T00:00:00Z");
    assert!(result.success);
    assert_eq!(result.hostname, Some("test-host".to_string()));
    assert_eq!(result.ip, Some("1.2.3.4".to_string()));
    assert_eq!(result.duration, 12345678);
    assert_eq!(result.errors.len(), 1);
    assert_eq!(result.status, Some(200));
    assert_eq!(result.condition_results.len(), 1);
    assert_eq!(result.condition_results[0].condition, "[STATUS] == 200");
    assert_eq!(result.body, Some("test-body".to_string()));
    assert_eq!(
        result
            .headers
            .as_ref()
            .unwrap()
            .get("Content-Type")
            .unwrap(),
        "text/plain"
    );
    assert_eq!(result.certificate_expiration, Some(3600));
}

#[test]
fn test_health_result_deserialization_minimal() {
    let raw_json = json!({
        "timestamp": "2023-01-01T00:00:00Z",
        "success": false,
        "duration": 100
    });

    let result: HealthResult = serde_json::from_value(raw_json).unwrap();

    assert_eq!(result.timestamp, "2023-01-01T00:00:00Z");
    assert!(!result.success);
    assert_eq!(result.duration, 100);
    assert!(result.errors.is_empty());
    assert!(result.condition_results.is_empty());
    assert!(result.body.is_none());
    assert!(result.headers.is_none());
}
