use gatus_mcp_rs::client::GatusClient;
use gatus_mcp_rs::mcp::McpHandler;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_handle_list_resources() {
    let client = GatusClient::new("http://localhost:8080".to_string(), None, None, None);
    let handler = McpHandler::new(client);

    let request = json!({
        "jsonrpc": "2.0",
        "method": "resources/list",
        "id": 1
    });

    let response = handler.handle(request).await;
    assert_eq!(response["id"], 1);

    let result = &response["result"];
    let resources = result["resources"]
        .as_array()
        .expect("resources should be an array");

    assert!(resources
        .iter()
        .any(|r| r["uri"] == "gatus://system/config"));
    assert!(resources
        .iter()
        .any(|r| r["uri"] == "gatus://dashboard/status"));
}

#[tokio::test]
async fn test_handle_read_resource_dashboard_status() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);
    let handler = McpHandler::new(client);

    let gatus_response = json!([
        {
            "name": "service-1",
            "group": "core",
            "status": "UP",
            "results": [
                {
                    "timestamp": "2026-04-16T10:00:00Z",
                    "success": true,
                    "duration": 100,
                    "conditionResults": []
                }
            ]
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/statuses"))
        .respond_with(ResponseTemplate::new(200).set_body_json(gatus_response))
        .mount(&mock_server)
        .await;

    let request = json!({
        "jsonrpc": "2.0",
        "method": "resources/read",
        "params": {
            "uri": "gatus://dashboard/status"
        },
        "id": 1
    });

    let response = handler.handle(request).await;
    assert_eq!(response["id"], 1);

    let result = &response["result"];
    if let Some(error) = response.get("error") {
        panic!("Response returned error: {:?}", error);
    }
    let contents = result["contents"]
        .as_array()
        .expect("contents should be an array");
    assert_eq!(contents.len(), 1);
    assert!(contents[0]["text"].as_str().unwrap().contains("UP"));
}

#[tokio::test]
async fn test_handle_read_resource_missing_uri() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);
    let handler = McpHandler::new(client);

    let request = json!({
        "jsonrpc": "2.0",
        "method": "resources/read",
        "params": {},
        "id": 1
    });

    let response = handler.handle(request).await;
    assert_eq!(response["error"]["code"], -32602);
    assert!(response["error"]["message"]
        .as_str()
        .unwrap()
        .contains("Missing 'uri' parameter"));
}

#[tokio::test]
async fn test_handle_read_resource_unknown_uri() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);
    let handler = McpHandler::new(client);

    let request = json!({
        "jsonrpc": "2.0",
        "method": "resources/read",
        "params": {
            "uri": "gatus://unknown"
        },
        "id": 1
    });

    let response = handler.handle(request).await;
    assert_eq!(response["error"]["code"], -32602);
    assert!(response["error"]["message"]
        .as_str()
        .unwrap()
        .contains("Unknown resource URI"));
}

#[tokio::test]
async fn test_handle_read_resource_system_config() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);
    let handler = McpHandler::new(client);

    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    let gatus_response = json!({"endpoints": []});

    Mock::given(method("GET"))
        .and(path("/api/v1/config"))
        .respond_with(ResponseTemplate::new(200).set_body_json(gatus_response))
        .mount(&mock_server)
        .await;

    let request = json!({
        "jsonrpc": "2.0",
        "method": "resources/read",
        "params": {
            "uri": "gatus://system/config"
        },
        "id": 1
    });

    let response = handler.handle(request).await;
    assert_eq!(response["id"], 1);
    let result = &response["result"];
    let contents = result["contents"]
        .as_array()
        .expect("contents should be an array");
    assert_eq!(contents.len(), 1);
    assert_eq!(contents[0]["mimeType"], "application/json");
}
