use gatus_mcp_rs::settings::Settings;

#[cfg(test)]
mod tests {
    use super::Settings;
    use std::env;

    #[test]
    fn test_settings_loading() {
        // We run these sequentially in one test to avoid environment variable race conditions

        // 1. Test Defaults
        env::remove_var("GATUS__SERVER__PORT");
        env::remove_var("GATUS__GATUS__API_URL");
        let settings = Settings::new().expect("Failed to load settings");
        assert_eq!(settings.server.port, 8080);
        assert_eq!(settings.server.host, "127.0.0.1");

        // 2. Test Environment Overrides
        env::set_var("GATUS__SERVER__PORT", "9090");
        env::set_var("GATUS__GATUS__API_URL", "http://gatus.example.com");
        let settings = Settings::new().expect("Failed to load settings");
        assert_eq!(settings.server.port, 9090);
        assert_eq!(settings.gatus.api_url, "http://gatus.example.com");

        // Cleanup
        env::remove_var("GATUS__SERVER__PORT");
        env::remove_var("GATUS__GATUS__API_URL");
    }
}
