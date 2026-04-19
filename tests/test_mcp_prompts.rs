use gatus_mcp_rs::client::GatusClient;
use gatus_mcp_rs::mcp::McpHandler;
use serde_json::json;
use wiremock::MockServer;

#[tokio::test]
async fn test_handle_list_prompts() {
    let client = GatusClient::new("http://localhost:8080".to_string(), None, None, None);
    let handler = McpHandler::new(client);

    let request = json!({
        "jsonrpc": "2.0",
        "method": "prompts/list",
        "id": 1
    });

    let response = handler.handle(request).await;
    assert_eq!(response["id"], 1);

    let result = &response["result"];
    let prompts = result["prompts"]
        .as_array()
        .expect("prompts should be an array");

    assert!(prompts.iter().any(|p| p["name"] == "analyze-outage"));
    assert!(prompts.iter().any(|p| p["name"] == "daily-health-report"));
}

#[tokio::test]
async fn test_handle_get_prompt_analyze_outage() {
    let client = GatusClient::new("http://localhost:8080".to_string(), None, None, None);
    let handler = McpHandler::new(client);

    let request = json!({
        "jsonrpc": "2.0",
        "method": "prompts/get",
        "params": {
            "name": "analyze-outage",
            "arguments": {
                "id": "my-service"
            }
        },
        "id": 1
    });

    let response = handler.handle(request).await;
    assert_eq!(response["id"], 1);

    let result = &response["result"];
    let messages = result["messages"]
        .as_array()
        .expect("messages should be an array");
    assert_eq!(messages.len(), 1);
    assert!(messages[0]["content"]["text"]
        .as_str()
        .unwrap()
        .contains("my-service"));
}

#[tokio::test]
async fn test_handle_get_prompt_missing_name() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);
    let handler = McpHandler::new(client);

    let request = json!({
        "jsonrpc": "2.0",
        "method": "prompts/get",
        "params": {},
        "id": 1
    });

    let response = handler.handle(request).await;
    assert_eq!(response["error"]["code"], -32602);
    assert!(response["error"]["message"]
        .as_str()
        .unwrap()
        .contains("Missing prompt name"));
}

#[tokio::test]
async fn test_handle_get_prompt_analyze_outage_missing_id() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);
    let handler = McpHandler::new(client);

    let request = json!({
        "jsonrpc": "2.0",
        "method": "prompts/get",
        "params": {
            "name": "analyze-outage",
            "arguments": {}
        },
        "id": 1
    });

    let response = handler.handle(request).await;
    assert_eq!(response["error"]["code"], -32602);
    assert!(response["error"]["message"]
        .as_str()
        .unwrap()
        .contains("Missing 'id' argument"));
}

#[tokio::test]
async fn test_handle_get_prompt_unknown_prompt() {
    let mock_server = MockServer::start().await;
    let client = GatusClient::new(mock_server.uri(), None, None, None);
    let handler = McpHandler::new(client);

    let request = json!({
        "jsonrpc": "2.0",
        "method": "prompts/get",
        "params": {
            "name": "unknown-prompt"
        },
        "id": 1
    });

    let response = handler.handle(request).await;
    assert_eq!(response["error"]["code"], -32601);
    assert!(response["error"]["message"]
        .as_str()
        .unwrap()
        .contains("Prompt not found"));
}
