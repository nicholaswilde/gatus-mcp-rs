use futures::StreamExt;
use gatus_mcp_rs::server::run_http_server;
use gatus_mcp_rs::settings::Settings;
use std::env;
use std::time::Duration;

#[tokio::test]
async fn test_live_sse_keepalive() {
    dotenvy::dotenv().ok();

    if env::var("GATUS_LIVE_TESTS").unwrap_or_default() != "true" {
        println!("Skipping live test: GATUS_LIVE_TESTS is not set to 'true'");
        return;
    }

    let api_url = env::var("GATUS_API_URL").expect("GATUS_API_URL must be set for live tests");
    let api_key = env::var("GATUS_API_KEY").ok();

    println!("Testing SSE against live Gatus: {}", api_url);

    let mut settings = Settings::new().unwrap();
    settings.gatus.api_url = api_url;
    settings.gatus.api_key = api_key;

    // Set polling to something long, but keep-alive to something short for the test
    // Actually, create_app and run_http_server use what's in settings.
    // Wait, create_app hardcodes keep-alive to 15s in server.rs.
    // I might want to modify server.rs to make it configurable if I want a fast test.
    // But for now, 15s is fine for a live test.

    let port = 8086;
    let host = "127.0.0.1".to_string();
    let server_settings = settings.clone();

    let server_handle = tokio::spawn(async move {
        run_http_server(server_settings, port, host).await.unwrap();
    });

    // Wait for server to start
    tokio::time::sleep(Duration::from_millis(500)).await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://127.0.0.1:{}/sse", port))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let mut stream = response.bytes_stream();

    println!("Waiting for SSE keep-alive (15s)...");
    let mut found_keepalive = false;
    let timeout = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(timeout);

    loop {
        tokio::select! {
            chunk = stream.next() => {
                if let Some(Ok(bytes)) = chunk {
                    let text = String::from_utf8_lossy(&bytes);
                    if text.contains(": keep-alive") {
                        println!("Received keep-alive!");
                        found_keepalive = true;
                        break;
                    }
                } else {
                    break;
                }
            }
            _ = &mut timeout => {
                println!("Timed out waiting for keep-alive");
                break;
            }
        }
    }

    server_handle.abort();
    assert!(found_keepalive, "Keep-alive not found in live SSE stream");
}
