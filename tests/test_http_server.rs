use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use gatus_mcp_rs::server::create_app;
use gatus_mcp_rs::settings::Settings;
use tower::ServiceExt; // for `oneshot`
use tokio_stream::StreamExt;

#[tokio::test]
async fn test_sse_endpoint() {
    let settings = Settings::new().unwrap();
    let app = create_app(settings);

    let response = app
        .oneshot(Request::builder().uri("/sse").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "text/event-stream"
    );

    let mut body = response.into_body().into_data_stream();
    let first_chunk = body.next().await.unwrap().unwrap();
    let first_text = String::from_utf8(first_chunk.to_vec()).unwrap();
    assert!(first_text.contains("data: ping"));
}

#[tokio::test]
async fn test_run_http_server() {
    let mut settings = Settings::new().unwrap();
    let port = 8081; // Use a different port
    let host = "127.0.0.1".to_string();
    settings.gatus.api_url = "http://localhost:8080".to_string();

    let server_handle = tokio::spawn(async move {
        gatus_mcp_rs::server::run_http_server(settings, port, host).await
    });

    // Wait a bit for server to start
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://127.0.0.1:{}/sse", port))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    server_handle.abort();
}

#[tokio::test]
async fn test_messages_endpoint() {
    let settings = Settings::new().unwrap();
    let app = create_app(settings);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/messages")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    r#"{"jsonrpc": "2.0", "method": "tools/list", "id": 1}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
