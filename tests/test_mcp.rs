use gatus_mcp_rs::client::GatusClient;
use gatus_mcp_rs::mcp::McpHandler;
use serde_json::json;

#[tokio::test]
async fn test_mcp_handler_unknown_tool() {
    let client = GatusClient::new("http://localhost".into(), None);
    let handler = McpHandler::new(client);
    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "unknown_tool",
            "arguments": {}
        },
        "id": 1
    });

    let response = handler.handle(request).await;
    assert_eq!(response["error"]["code"], -32601); // Method not found
}

#[tokio::test]
async fn test_mcp_handler_list_tools() {
    let client = GatusClient::new("http://localhost".into(), None);
    let handler = McpHandler::new(client);
    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/list",
        "params": {},
        "id": 1
    });

    let response = handler.handle(request).await;
    assert!(response["result"]["tools"].is_array());
    assert!(response["result"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .any(|t| t["name"] == "manage_resources"));
    assert!(response["result"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .any(|t| t["name"] == "get_metrics"));
}
