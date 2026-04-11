use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use moka::future::Cache;
use std::time::Duration;
use governor::{Quota, RateLimiter};
use std::num::NonZeroU32;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EndpointStatus {
    pub name: String,
    pub group: String,
    pub status: String,
    #[serde(default)]
    pub results: Vec<HealthResult>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HealthResult {
    pub timestamp: String,
    pub success: bool,
    pub hostname: String,
    pub ip: String,
    pub duration: u64,
    pub errors: Vec<String>,
    pub status: u16,
}

pub struct GatusClient {
    api_url: String,
    api_key: Option<String>,
    client: Client,
    cache: Cache<String, Vec<EndpointStatus>>,
    rate_limiter: RateLimiter<governor::state::NotKeyed, governor::state::InMemoryState, governor::clock::DefaultClock>,
}

impl GatusClient {
    pub fn new(api_url: String, api_key: Option<String>) -> Self {
        let quota = Quota::per_second(NonZeroU32::new(2).unwrap()); // 2 requests per second
        
        Self {
            api_url,
            api_key,
            client: Client::new(),
            cache: Cache::builder()
                .max_capacity(100)
                .time_to_live(Duration::from_secs(60))
                .build(),
            rate_limiter: RateLimiter::direct(quota),
        }
    }

    pub async fn list_services(&self) -> Result<Vec<EndpointStatus>> {
        let cache_key = "endpoints_statuses".to_string();
        
        if let Some(cached) = self.cache.get(&cache_key).await {
            return Ok(cached);
        }

        self.rate_limiter.until_ready().await;

        let url = format!("{}/api/v1/endpoints/statuses", self.api_url);
        let mut request = self.client.get(url);

        if let Some(ref key) = self.api_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }

        let response = request.send().await?;
        let status = response.status();
        let text = response.text().await.context("Failed to get response text")?;

        if !status.is_success() {
            anyhow::bail!("Gatus API error: status {}, body: {}", status, text);
        }

        let services: Vec<EndpointStatus> = serde_json::from_str(&text)
            .with_context(|| format!("Failed to decode Gatus API response: {}", text))?;
            
        self.cache.insert(cache_key, services.clone()).await;
        
        Ok(services)
    }
}
