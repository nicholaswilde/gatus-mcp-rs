use gatus_mcp_rs::client::GatusClient;
use gatus_mcp_rs::mcp::McpHandler;
use serde_json::json;
use std::env;

#[tokio::test]
async fn test_live_get_metrics() {
    dotenvy::dotenv().ok();

    if env::var("GATUS_LIVE_TESTS").unwrap_or_default() != "true" {
        println!("Skipping live test: GATUS_LIVE_TESTS is not set to 'true'");
        return;
    }

    let api_url = env::var("GATUS_API_URL").expect("GATUS_API_URL must be set for live tests");
    let api_key = env::var("GATUS_API_KEY").ok();

    println!("Testing get_metrics against live instance: {}", api_url);

    let client = std::sync::Arc::new(GatusClient::new(api_url, api_key, None, None));
    let handler = McpHandler::new_with_arc(client.clone());

    // 1. System stats
    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "get_metrics",
            "arguments": {
                "action": "system-stats"
            }
        },
        "id": 1
    });

    let response = handler.handle(request).await;
    assert!(
        response["error"].is_null(),
        "Expected no error for system-stats, got: {:?}",
        response["error"]
    );
    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("**Total Endpoints:**"));

    // 2. Alert History
    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "get_metrics",
            "arguments": {
                "action": "alert-history",
                "limit": 3
            }
        },
        "id": 2
    });

    let response = handler.handle(request).await;
    assert!(
        response["error"].is_null(),
        "Expected no error for alert-history, got: {:?}",
        response["error"]
    );
    // (Output might be empty if no alerts in Gatus)

    // 3. Service specific actions - Need to get a real service name first
    let services = client
        .list_services(true)
        .await
        .expect("Failed to list services");
    if let Some(service) = services.first() {
        println!("Testing service-specific metrics for: {}", service.name);

        // service-details
        let request = json!({
            "jsonrpc": "2.0",
            "method": "tools/call",
            "params": {
                "name": "get_metrics",
                "arguments": {
                    "action": "service-details",
                    "id": &service.name
                }
            },
            "id": 3
        });
        let response = handler.handle(request).await;
        assert!(response["error"].is_null());
        let text = response["result"]["content"][0]["text"].as_str().unwrap();
        assert!(text.contains(&service.name));

        // uptime
        let request = json!({
            "jsonrpc": "2.0",
            "method": "tools/call",
            "params": {
                "name": "get_metrics",
                "arguments": {
                    "action": "uptime",
                    "id": &service.name,
                    "timeframe": "24h"
                }
            },
            "id": 4
        });
        let response = handler.handle(request).await;
        assert!(response["error"].is_null());
        let text = response["result"]["content"][0]["text"].as_str().unwrap();
        assert!(text.contains("%"));

        // group-summary
        let request = json!({
            "jsonrpc": "2.0",
            "method": "tools/call",
            "params": {
                "name": "get_metrics",
                "arguments": {
                    "action": "group-summary",
                    "id": &service.group
                }
            },
            "id": 5
        });
        let response = handler.handle(request).await;
        assert!(response["error"].is_null());
        let text = response["result"]["content"][0]["text"].as_str().unwrap();
        assert!(text.contains(&service.group));
    } else {
        println!("No services found, skipping service-specific tests");
    }
}
