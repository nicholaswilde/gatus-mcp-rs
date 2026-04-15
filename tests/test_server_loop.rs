use gatus_mcp_rs::client::GatusClient;
use gatus_mcp_rs::mcp::McpHandler;
use gatus_mcp_rs::server::run_server_loop;
use std::io::Cursor;

#[tokio::test]
async fn test_run_server_loop() {
    let gatus_client = GatusClient::new("http://localhost:8080".to_string(), Some("api_key".to_string()));
    let handler = McpHandler::new(gatus_client);

    let input = r#"{"jsonrpc": "2.0", "method": "initialize", "params": {}, "id": 1}"#;
    let reader = Cursor::new(input.as_bytes());
    let mut writer = Cursor::new(Vec::new());

    run_server_loop(handler, reader, &mut writer).await.unwrap();

    let output = String::from_utf8(writer.into_inner()).unwrap();
    assert!(output.contains(r#""jsonrpc":"2.0""#));
    assert!(output.contains(r#""id":1"#));
    assert!(output.contains(r#""protocolVersion":"2024-11-05""#));
}

#[tokio::test]
async fn test_run_server_loop_invalid_json() {
    let gatus_client = GatusClient::new("http://localhost:8080".to_string(), Some("api_key".to_string()));
    let handler = McpHandler::new(gatus_client);

    let input = "invalid json\n";
    let reader = Cursor::new(input.as_bytes());
    let mut writer = Cursor::new(Vec::new());

    run_server_loop(handler, reader, &mut writer).await.unwrap();

    let output = String::from_utf8(writer.into_inner()).unwrap();
    assert!(output.is_empty());
}
