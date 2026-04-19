use anyhow::{Context, Result};
use chrono::{DateTime, Duration as ChronoDuration, Utc};
use governor::{Quota, RateLimiter};
use moka::future::Cache;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EndpointStatus {
    pub name: String,
    pub group: String,
    pub status: Option<String>,
    #[serde(default)]
    pub results: Vec<HealthResult>,
    #[serde(default)]
    pub events: Vec<Event>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Event {
    #[serde(rename = "type")]
    pub event_type: String,
    pub timestamp: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AlertEvent {
    pub service: String,
    pub group: String,
    pub event_type: String,
    pub timestamp: String,
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

    pub fn calculate_uptime(&self, timeframe: &str) -> f64 {
        if self.results.is_empty() {
            return 100.0;
        }

        let now = Utc::now();
        let duration = match timeframe {
            "7d" => ChronoDuration::days(7),
            "30d" => ChronoDuration::days(30),
            _ => ChronoDuration::hours(24),
        };
        let cutoff = now - duration;

        let filtered_results: Vec<&HealthResult> = self
            .results
            .iter()
            .filter(|r| {
                if let Ok(ts) = DateTime::parse_from_rfc3339(&r.timestamp) {
                    ts.with_timezone(&Utc) >= cutoff
                } else {
                    false
                }
            })
            .collect();

        if filtered_results.is_empty() {
            return 100.0;
        }

        let success_count = filtered_results.iter().filter(|r| r.success).count();
        (success_count as f64 / filtered_results.len() as f64) * 100.0
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
    pub body: Option<String>,
    pub headers: Option<std::collections::HashMap<String, String>>,
    pub certificate_expiration: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConditionResult {
    pub condition: String,
    pub success: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ResponseTimePoint {
    pub timestamp: String,
    pub value: u64,
}

pub type UptimeResponse = std::collections::HashMap<String, f64>;
pub type ResponseTimeResponse = Vec<ResponseTimePoint>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemStats {
    pub total: usize,
    pub up: usize,
    pub down: usize,
    pub degraded: usize,
    pub certificates_expiring_soon: usize,
}

#[derive(Clone)]
pub struct GatusClient {
    api_url: String,
    api_key: Option<String>,
    client: Client,
    cache: Cache<String, Vec<EndpointStatus>>,
    uptime_cache: Cache<String, UptimeResponse>,
    response_time_cache: Cache<String, ResponseTimeResponse>,
    rate_limiter: Arc<
        RateLimiter<
            governor::state::NotKeyed,
            governor::state::InMemoryState,
            governor::clock::DefaultClock,
        >,
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
            uptime_cache: Cache::builder()
                .max_capacity(100)
                .time_to_live(Duration::from_secs(60))
                .build(),
            response_time_cache: Cache::builder()
                .max_capacity(100)
                .time_to_live(Duration::from_secs(60))
                .build(),
            rate_limiter: Arc::new(RateLimiter::direct(quota)),
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_endpoint_statuses(&self, key: &str) -> Result<Vec<EndpointStatus>> {
        self.rate_limiter.until_ready().await;

        let url = format!("{}/api/v1/endpoints/{}/statuses", self.api_url, key);
        let mut request = self.client.get(url);

        if let Some(ref key) = self.api_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }

        let response = request.send().await?;
        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("Gatus API error: status {}, body: {}", status, text);
        }

        let services: Vec<EndpointStatus> = serde_json::from_str(&text)?;
        Ok(services)
    }

    #[tracing::instrument(skip(self))]
    pub async fn list_services(&self, refresh: bool) -> Result<Vec<EndpointStatus>> {
        let cache_key = "endpoints_statuses".to_string();

        if !refresh {
            if let Some(cached) = self.cache.get(&cache_key).await {
                return Ok(cached);
            }
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
        let services = self.list_services(false).await?;
        let mut up = 0;
        let mut down = 0;
        let mut degraded = 0;
        let mut certificates_expiring_soon = 0;

        // 30 days in nanoseconds
        let threshold = 30 * 24 * 60 * 60 * 1_000_000_000u64;

        for service in &services {
            let status = service.display_status().to_uppercase();
            match status.as_str() {
                "UP" => up += 1,
                "DOWN" => down += 1,
                "DEGRADED" => degraded += 1,
                _ => {
                    if let Some(result) = service.results.first() {
                        if result.success {
                            up += 1;
                        } else {
                            down += 1;
                        }
                    }
                }
            }

            if let Some(result) = service.results.first() {
                if let Some(exp) = result.certificate_expiration {
                    if exp < threshold {
                        certificates_expiring_soon += 1;
                    }
                }
            }
        }

        Ok(SystemStats {
            total: services.len(),
            up,
            down,
            degraded,
            certificates_expiring_soon,
        })
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_uptime(&self, service_name: &str, timeframe: &str) -> Result<f64> {
        let services = self.list_services(false).await?;
        let service = services
            .iter()
            .find(|s| s.name == service_name)
            .ok_or_else(|| anyhow::anyhow!("Service not found: {}", service_name))?;

        Ok(service.calculate_uptime(timeframe))
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_alert_history(&self, limit: usize) -> Result<Vec<AlertEvent>> {
        let services = self.list_services(false).await?;
        let mut all_events = Vec::new();

        for service in services {
            for event in service.events {
                all_events.push(AlertEvent {
                    service: service.name.clone(),
                    group: service.group.clone(),
                    event_type: event.event_type,
                    timestamp: event.timestamp,
                });
            }
        }

        // Sort events by timestamp descending (newest first)
        all_events.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        Ok(all_events.into_iter().take(limit).collect())
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_endpoint_uptimes(&self, key: &str, duration: &str) -> Result<UptimeResponse> {
        let cache_key = format!("uptimes_{}_{}", key, duration);
        if let Some(cached) = self.uptime_cache.get(&cache_key).await {
            return Ok(cached);
        }

        self.rate_limiter.until_ready().await;

        let url = format!(
            "{}/api/v1/endpoints/{}/uptimes/{}",
            self.api_url, key, duration
        );
        let mut request = self.client.get(url);

        if let Some(ref key) = self.api_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }

        let response = request.send().await?;
        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("Gatus API error: status {}, body: {}", status, text);
        }

        let uptimes: UptimeResponse = serde_json::from_str(&text)?;
        self.uptime_cache.insert(cache_key, uptimes.clone()).await;
        Ok(uptimes)
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_endpoint_response_times(
        &self,
        key: &str,
        duration: &str,
    ) -> Result<ResponseTimeResponse> {
        let cache_key = format!("response_times_{}_{}", key, duration);
        if let Some(cached) = self.response_time_cache.get(&cache_key).await {
            return Ok(cached);
        }

        self.rate_limiter.until_ready().await;

        let url = format!(
            "{}/api/v1/endpoints/{}/response-times/{}",
            self.api_url, key, duration
        );
        let mut request = self.client.get(url);

        if let Some(ref key) = self.api_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }

        let response = request.send().await?;
        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("Gatus API error: status {}, body: {}", status, text);
        }

        let response_times: ResponseTimeResponse = serde_json::from_str(&text)?;
        self.response_time_cache
            .insert(cache_key, response_times.clone())
            .await;
        Ok(response_times)
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_config(&self) -> Result<Value> {
        self.rate_limiter.until_ready().await;

        let url = format!("{}/api/v1/config", self.api_url);
        let mut request = self.client.get(url);

        if let Some(ref key) = self.api_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }

        let response = request.send().await?;
        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("Gatus API error: status {}, body: {}", status, text);
        }

        let config: Value = serde_json::from_str(&text)?;
        Ok(config)
    }

    #[tracing::instrument(skip(self))]
    pub async fn trigger_check(&self, key: &str) -> Result<()> {
        self.rate_limiter.until_ready().await;

        let url = format!("{}/api/v1/endpoints/{}/trigger", self.api_url, key);
        let mut request = self.client.post(url);

        if let Some(ref key) = self.api_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }

        let response = request.send().await?;
        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("Gatus API error: status {}, body: {}", status, text);
        }

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn reload_config(&self) -> Result<()> {
        self.rate_limiter.until_ready().await;

        let url = format!("{}/api/v1/config/reload", self.api_url);
        let mut request = self.client.post(url);

        if let Some(ref key) = self.api_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }

        let response = request.send().await?;
        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("Gatus API error: status {}, body: {}", status, text);
        }

        Ok(())
    }

    pub fn get_badge_url(&self, key: &str) -> String {
        format!("{}/api/v1/endpoints/{}/health/badge.svg", self.api_url, key)
    }

    pub fn get_uptime_badge_url(&self, key: &str, duration: &str) -> String {
        format!(
            "{}/api/v1/endpoints/{}/uptimes/{}/badge.svg",
            self.api_url, key, duration
        )
    }

    pub fn get_latency_badge_url(&self, key: &str, duration: &str) -> String {
        format!(
            "{}/api/v1/endpoints/{}/response-times/{}/badge.svg",
            self.api_url, key, duration
        )
    }

    pub fn get_latency_chart_url(&self, key: &str, duration: &str) -> String {
        format!(
            "{}/api/v1/endpoints/{}/response-times/{}/chart.svg",
            self.api_url, key, duration
        )
    }

    #[tracing::instrument(skip(self, result))]
    pub async fn push_endpoint_result(&self, key: &str, result: HealthResult) -> Result<()> {
        self.rate_limiter.until_ready().await;

        let url = format!("{}/api/v1/endpoints/{}/external", self.api_url, key);
        let mut query_params = vec![
            ("success", result.success.to_string()),
            ("duration", result.duration.to_string()),
        ];
        if let Some(err) = result.errors.first() {
            query_params.push(("error", err.clone()));
        }

        let mut request = self.client.post(url).query(&query_params);

        if let Some(ref key) = self.api_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }

        let response = request.send().await?;
        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("Gatus API error: status {}, body: {}", status, text);
        }

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_instance_health(&self) -> Result<String> {
        self.rate_limiter.until_ready().await;

        let url = format!("{}/health", self.api_url);
        let mut request = self.client.get(url);

        if let Some(ref key) = self.api_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }

        let response = request.send().await?;
        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("Gatus API error: status {}, body: {}", status, text);
        }

        Ok(text)
    }
}
