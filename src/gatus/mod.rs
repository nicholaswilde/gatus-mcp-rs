use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct EndpointStatus {
    pub name: String,
    pub group: String,
    pub status: String,
    #[serde(default)]
    pub results: Vec<HealthResult>,
}

#[derive(Debug, Deserialize, Clone)]
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
}

impl GatusClient {
    pub fn new(api_url: String, api_key: Option<String>) -> Self {
        Self {
            api_url,
            api_key,
            client: Client::new(),
        }
    }

    pub async fn list_services(&self) -> Result<Vec<EndpointStatus>> {
        let url = format!("{}/api/v1/endpoints/statuses", self.api_url);
        let mut request = self.client.get(url);

        if let Some(ref key) = self.api_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }

        let response = request.send().await?.json::<Vec<EndpointStatus>>().await?;
        Ok(response)
    }
}
