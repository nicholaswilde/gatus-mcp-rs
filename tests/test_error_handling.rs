use gatus_mcp_rs::client::GatusClient;
use gatus_mcp_rs::mcp::McpHandler;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_mcp_handler_api_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/statuses"))
        .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
        .mount(&mock_server)
        .await;

    let client = GatusClient::new(mock_server.uri(), None);
    let handler = McpHandler::new(client);

    let req = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "manage_resources",
            "arguments": {
                "action": "list-services"
            }
        },
        "id": 1
    });

    let resp = handler.handle(req).await;

    assert_eq!(resp["id"], 1);
    assert_eq!(resp["error"]["code"], -32000);
    assert!(resp["error"]["message"]
        .as_str()
        .unwrap()
        .contains("Gatus API error"));
}

#[tokio::test]
async fn test_mcp_handler_service_not_found() {
    let mock_server = MockServer::start().await;

    // Return empty list
    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/statuses"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
        .mount(&mock_server)
        .await;

    let client = GatusClient::new(mock_server.uri(), None);
    let handler = McpHandler::new(client);

    let req = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "get_metrics",
            "arguments": {
                "id": "non-existent",
                "action": "service-details"
            }
        },
        "id": 1
    });

    let resp = handler.handle(req).await;

    assert_eq!(resp["id"], 1);
    assert_eq!(resp["error"]["code"], -32602);
    assert!(resp["error"]["message"]
        .as_str()
        .unwrap()
        .contains("Service 'non-existent' not found"));
}

#[tokio::test]
async fn test_mcp_handler_invalid_request() {
    let client = GatusClient::new("http://localhost".to_string(), None);
    let handler = McpHandler::new(client);

    let req = json!("invalid");
    let resp = handler.handle(req).await;

    assert_eq!(resp["error"]["code"], -32600);
    assert_eq!(resp["error"]["message"], "Invalid Request");
}

#[tokio::test]
async fn test_mcp_handler_missing_tool_name() {
    let client = GatusClient::new("http://localhost".to_string(), None);
    let handler = McpHandler::new(client);

    let req = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "arguments": {}
        },
        "id": 1
    });

    let resp = handler.handle(req).await;

    assert_eq!(resp["error"]["code"], -32602);
    assert_eq!(resp["error"]["message"], "Missing tool name");
}

#[tokio::test]
async fn test_mcp_handler_unknown_manage_resources_action() {
    let client = GatusClient::new("http://localhost".to_string(), None);
    let handler = McpHandler::new(client);

    let req = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "manage_resources",
            "arguments": {
                "action": "unknown"
            }
        },
        "id": 1
    });

    let resp = handler.handle(req).await;

    assert_eq!(resp["error"]["code"], -32602);
    assert!(resp["error"]["message"]
        .as_str()
        .unwrap()
        .contains("Unknown action 'unknown'"));
}

#[tokio::test]
async fn test_mcp_handler_get_metrics_missing_action() {
    let client = GatusClient::new("http://localhost".to_string(), None);
    let handler = McpHandler::new(client);

    let req = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "get_metrics",
            "arguments": {}
        },
        "id": 1
    });

    let resp = handler.handle(req).await;

    assert_eq!(resp["error"]["code"], -32602);
    assert_eq!(resp["error"]["message"], "Missing 'action' argument");
}

#[tokio::test]
async fn test_gatus_client_api_error_uptimes() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;

    let client = GatusClient::new(mock_server.uri(), None);
    let res = client.get_endpoint_uptimes("key", "24h").await;

    assert!(res.is_err());
}

#[tokio::test]
async fn test_gatus_client_api_error_response_times() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;

    let client = GatusClient::new(mock_server.uri(), None);
    let res = client.get_endpoint_response_times("key", "24h").await;

    assert!(res.is_err());
}

#[tokio::test]
async fn test_gatus_client_api_error_health() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;

    let client = GatusClient::new(mock_server.uri(), None);
    let res = client.get_instance_health().await;

    assert!(res.is_err());
}

#[tokio::test]
async fn test_mcp_handler_get_config_error() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/statuses"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;

    let client = GatusClient::new(mock_server.uri(), None);
    let handler = McpHandler::new(client);

    let req = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "manage_resources",
            "arguments": { "action": "get-config" }
        },
        "id": 1
    });

    let resp = handler.handle(req).await;
    assert_eq!(resp["error"]["code"], -32000);
}

#[tokio::test]
async fn test_mcp_handler_get_health_error() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/health"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;

    let client = GatusClient::new(mock_server.uri(), None);
    let handler = McpHandler::new(client);

    let req = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "manage_resources",
            "arguments": { "action": "get-health" }
        },
        "id": 1
    });

    let resp = handler.handle(req).await;
    assert_eq!(resp["error"]["code"], -32000);
}

#[tokio::test]
async fn test_mcp_handler_system_stats_error() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/statuses"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;

    let client = GatusClient::new(mock_server.uri(), None);
    let handler = McpHandler::new(client);

    let req = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "get_metrics",
            "arguments": { "action": "system-stats" }
        },
        "id": 1
    });

    let resp = handler.handle(req).await;
    assert_eq!(resp["error"]["code"], -32000);
}

#[tokio::test]
async fn test_mcp_handler_uptime_granular_error() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/statuses"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {"name": "svc", "group": "grp", "results": []}
        ])))
        .mount(&mock_server)
        .await;

    let client = GatusClient::new(mock_server.uri(), None);
    let handler = McpHandler::new(client);

    let req = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "get_metrics",
            "arguments": { "action": "uptime-granular", "id": "grp_svc" }
        },
        "id": 1
    });

    let resp = handler.handle(req).await;
    assert_eq!(resp["error"]["code"], -32000);
}

#[tokio::test]
async fn test_mcp_handler_response_time_error() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/statuses"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {"name": "svc", "group": "grp", "results": []}
        ])))
        .mount(&mock_server)
        .await;

    let client = GatusClient::new(mock_server.uri(), None);
    let handler = McpHandler::new(client);

    let req = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "get_metrics",
            "arguments": { "action": "response-time", "id": "grp_svc" }
        },
        "id": 1
    });

    let resp = handler.handle(req).await;
    assert_eq!(resp["error"]["code"], -32000);
}

#[tokio::test]
async fn test_mcp_handler_alert_history_error() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/statuses"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;

    let client = GatusClient::new(mock_server.uri(), None);
    let handler = McpHandler::new(client);

    let req = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "get_metrics",
            "arguments": { "action": "alert-history" }
        },
        "id": 1
    });

    let resp = handler.handle(req).await;
    assert_eq!(resp["error"]["code"], -32000);
}
