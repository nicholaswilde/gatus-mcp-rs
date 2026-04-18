use anyhow::Result;
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerSettings {
    pub port: u16,
    pub host: String,
    pub polling_interval: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GatusSettings {
    pub api_url: String,
    pub api_key: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Settings {
    pub server: ServerSettings,
    pub gatus: GatusSettings,
}

impl Default for ServerSettings {
    fn default() -> Self {
        Self {
            port: 8080,
            host: "127.0.0.1".to_string(),
            polling_interval: 60,
        }
    }
}

impl Default for GatusSettings {
    fn default() -> Self {
        Self {
            api_url: "http://localhost:8080".to_string(),
            api_key: None,
        }
    }
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        // Load .env file if it exists
        dotenvy::dotenv().ok();

        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let mut builder = Config::builder()
            // Start with default values
            .set_default("server.port", 8080)?
            .set_default("server.host", "127.0.0.1")?
            .set_default("server.polling_interval", 60)?
            .set_default("gatus.api_url", "http://localhost:8080")?
            // Add local config file (optional)
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            .add_source(File::with_name("config/local").required(false))
            // Support flatter environment variables from .env.example
            .add_source(Environment::with_prefix("GATUS").separator("_"))
            // Also support the double-underscore separator for standard config mapping
            .add_source(Environment::with_prefix("GATUS").separator("__"));

        // Manual overrides for conventional env vars from .env.example
        if let Ok(url) = env::var("GATUS_API_URL") {
            builder = builder.set_override("gatus.api_url", url)?;
        }
        if let Ok(key) = env::var("GATUS_API_KEY") {
            builder = builder.set_override("gatus.api_key", key)?;
        }

        builder.build()?.try_deserialize()
    }
}
