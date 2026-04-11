use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt; // for `oneshot`
use gatus_mcp_rs::server::create_app;
use gatus_mcp_rs::settings::Settings;

#[tokio::test]
async fn test_sse_endpoint() {
    let settings = Settings::new().unwrap();
    let app = create_app(settings);

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
    let app = create_app(settings);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/messages")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"jsonrpc": "2.0", "method": "tools/list", "id": 1}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
