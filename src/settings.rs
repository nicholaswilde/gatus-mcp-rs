use anyhow::Result;
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerSettings {
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GatusSettings {
    pub api_url: String,
    pub api_key: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: ServerSettings,
    pub gatus: GatusSettings,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        // Load .env file if it exists
        dotenvy::dotenv().ok();

        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            // Start with default values
            .set_default("server.port", 8080)?
            .set_default("server.host", "127.0.0.1")?
            .set_default("gatus.api_url", "http://localhost:8080")?
            // Add local config file (optional)
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            .add_source(File::with_name("config/local").required(false))
            // Add environment variables
            // GATUS__SERVER__PORT maps to server.port
            // GATUS__GATUS__API_URL maps to gatus.api_url
            .add_source(Environment::with_prefix("GATUS").separator("__"))
            // Also support a flatter env var for the base URL specifically if needed
            .build()?;

        s.try_deserialize()
    }
}
