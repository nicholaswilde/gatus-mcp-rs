use gatus_mcp_rs::settings::Settings;

#[cfg(test)]
mod tests {
    use super::Settings;
    use std::env;

    #[test]
    fn test_settings_loading() {
        // We run these sequentially in one test to avoid environment variable race conditions

        // Clear environment to avoid interference from .env
        env::remove_var("GATUS_API_URL");
        env::remove_var("GATUS_API_KEY");
        env::remove_var("GATUS_SERVER_PORT");
        env::remove_var("GATUS__SERVER__PORT");
        env::remove_var("GATUS__GATUS__API_URL");

        // 1. Test Defaults
        env::remove_var("GATUS__SERVER__PORT");
        env::remove_var("GATUS__GATUS__API_URL");
        let settings = Settings::new().expect("Failed to load settings");
        assert_eq!(settings.server.port, 8080);
        assert_eq!(settings.server.host, "127.0.0.1");

        // 2. Test Environment Overrides
        env::set_var("GATUS__SERVER__PORT", "9090");
        env::set_var("GATUS__GATUS__API_URL", "http://gatus.example.com");
        // We also set GATUS_API_URL because it's now an override that would trump GATUS__GATUS__API_URL
        env::set_var("GATUS_API_URL", "http://gatus.example.com");
        let settings = Settings::new().expect("Failed to load settings");
        assert_eq!(settings.server.port, 9090);
        assert_eq!(settings.gatus.api_url, "http://gatus.example.com");

        // 3. Test Conventional Environment Variables (.env.example style)
        env::set_var("GATUS_API_URL", "http://conventional.example.com");
        env::set_var("GATUS_API_KEY", "super-secret-key");
        let settings = Settings::new().expect("Failed to load settings");
        assert_eq!(settings.gatus.api_url, "http://conventional.example.com");
        assert_eq!(settings.gatus.api_key, Some("super-secret-key".into()));

        // Cleanup
        env::remove_var("GATUS__SERVER__PORT");
        env::remove_var("GATUS__GATUS__API_URL");
        env::remove_var("GATUS_API_URL");
        env::remove_var("GATUS_API_KEY");
    }
}
