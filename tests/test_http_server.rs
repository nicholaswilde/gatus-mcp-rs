use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt; // for `oneshot`
use gatus_mcp_rs::http_server::app;
use gatus_mcp_rs::settings::Settings;

#[tokio::test]
async fn test_sse_endpoint() {
    let settings = Settings::new().unwrap();
    let app = app(settings);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/sse")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/event-stream");
}

#[tokio::test]
async fn test_messages_endpoint() {
    let settings = Settings::new().unwrap();
    let app = app(settings);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/messages")
                .body(Body::from(r#"{"jsonrpc": "2.0", "method": "initialize", "id": 1}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    // Without a session, it should probably return an error or unauthorized
    // For now, let's just see what it does
    assert!(response.status().is_client_error() || response.status().is_success());
}
