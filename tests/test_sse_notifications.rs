use gatus_mcp_rs::settings::{Settings, GatusSettings, ServerSettings};
use gatus_mcp_rs::server::run_http_server;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};
use std::time::Duration;
use futures::StreamExt;

#[tokio::test]
async fn test_sse_notifications() {
    let mock_server = MockServer::start().await;
    
    // Initial response
    let gatus_response_1 = json!([
        {
            "name": "service-1",
            "group": "core",
            "status": "UP",
            "results": []
        }
    ]);

    // Changed response
    let gatus_response_2 = json!([
        {
            "name": "service-1",
            "group": "core",
            "status": "DOWN",
            "results": []
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/statuses"))
        .respond_with(ResponseTemplate::new(200).set_body_json(gatus_response_1))
        .up_to_n_times(1)
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/statuses"))
        .respond_with(ResponseTemplate::new(200).set_body_json(gatus_response_2))
        .mount(&mock_server)
        .await;

    let port = 8085;
    let host = "127.0.0.1".to_string();
    let settings = Settings {
        server: ServerSettings {
            port,
            host: host.clone(),
            polling_interval: 1, // 1 second for test
        },
        gatus: GatusSettings {
            api_url: mock_server.uri(),
            api_key: None,
        },
    };

    let server_settings = settings.clone();
    let server_handle = tokio::spawn(async move {
        run_http_server(server_settings, port, host).await.unwrap();
    });

    // Wait for server to start
    tokio::time::sleep(Duration::from_millis(500)).await;

    let client = reqwest::Client::new();
    let mut response = client
        .get(format!("http://127.0.0.1:{}/sse", port))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let mut stream = response.bytes_stream();
    
    // We expect a notification soon
    let mut found_notification = false;
    let timeout = tokio::time::sleep(Duration::from_secs(5));
    tokio::pin!(timeout);

    loop {
        tokio::select! {
            chunk = stream.next() => {
                if let Some(Ok(bytes)) = chunk {
                    let text = String::from_utf8_lossy(&bytes);
                    if text.contains("notifications/resources/updated") && text.contains("is now DOWN") {
                        found_notification = true;
                        break;
                    }
                } else {
                    break;
                }
            }
            _ = &mut timeout => {
                break;
            }
        }
    }

    server_handle.abort();
    assert!(found_notification, "Notification not found in SSE stream");
}
