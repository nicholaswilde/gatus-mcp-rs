use anyhow::{Context, Result};
use governor::{Quota, RateLimiter};
use moka::future::Cache;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;
use std::time::Duration;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EndpointStatus {
    pub name: String,
    pub group: String,
    pub status: Option<String>,
    #[serde(default)]
    pub results: Vec<HealthResult>,
}

impl EndpointStatus {
    pub fn display_status(&self) -> String {
        self.status
            .clone()
            .unwrap_or_else(|| match self.results.first() {
                Some(r) => {
                    if r.success {
                        "UP".to_string()
                    } else {
                        "DOWN".to_string()
                    }
                }
                None => "UNKNOWN".to_string(),
            })
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HealthResult {
    pub timestamp: String,
    pub success: bool,
    pub hostname: Option<String>,
    pub ip: Option<String>,
    pub duration: u64,
    #[serde(default)]
    pub errors: Vec<String>,
    pub status: Option<u16>,
    #[serde(rename = "conditionResults", default)]
    pub condition_results: Vec<ConditionResult>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConditionResult {
    pub condition: String,
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemStats {
    pub total: usize,
    pub up: usize,
    pub down: usize,
    pub degraded: usize,
}

pub struct GatusClient {
    api_url: String,
    api_key: Option<String>,
    client: Client,
    cache: Cache<String, Vec<EndpointStatus>>,
    rate_limiter: RateLimiter<
        governor::state::NotKeyed,
        governor::state::InMemoryState,
        governor::clock::DefaultClock,
    >,
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

    #[tracing::instrument(skip(self))]
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
        let text = response
            .text()
            .await
            .context("Failed to get response text")?;

        if !status.is_success() {
            anyhow::bail!("Gatus API error: status {}, body: {}", status, text);
        }

        let services: Vec<EndpointStatus> = serde_json::from_str(&text)
            .with_context(|| format!("Failed to decode Gatus API response: {}", text))?;

        self.cache.insert(cache_key, services.clone()).await;

        Ok(services)
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_system_stats(&self) -> Result<SystemStats> {
        let services = self.list_services().await?;
        let mut up = 0;
        let mut down = 0;
        let mut degraded = 0;

        for service in &services {
            let status = service.display_status().to_uppercase();
            match status.as_str() {
                "UP" => up += 1,
                "DOWN" => down += 1,
                "DEGRADED" => degraded += 1,
                _ => {
                    tracing::debug!("Service {} has unknown status: {}", service.name, status);
                }
            }
        }

        Ok(SystemStats {
            total: services.len(),
            up,
            down,
            degraded,
        })
    }
}
