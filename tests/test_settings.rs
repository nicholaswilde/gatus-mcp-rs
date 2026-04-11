use std::env;
// Importing from the project's library structure (once we define it)
// For now, we'll use the module-in-test pattern since it's a small project
#[path = "../src/settings.rs"]
mod settings;

#[cfg(test)]
mod tests {
    use super::settings::Settings;
    use std::env;

    #[test]
    fn test_load_settings_from_env() {
        // Clear environment variables to ensure a clean state
        env::remove_var("GATUS__SERVER__PORT");
        env::remove_var("GATUS__GATUS__API_URL");
        
        env::set_var("GATUS__SERVER__PORT", "9090");
        env::set_var("GATUS__GATUS__API_URL", "http://gatus.example.com");
        
        let settings = Settings::new().expect("Failed to load settings");
        
        assert_eq!(settings.server.port, 9090);
        assert_eq!(settings.gatus.api_url, "http://gatus.example.com");
    }

    #[test]
    fn test_load_settings_defaults() {
        env::remove_var("GATUS__SERVER__PORT");
        env::remove_var("GATUS__GATUS__API_URL");
        
        let settings = Settings::new().expect("Failed to load settings");
        
        assert_eq!(settings.server.port, 8080);
        assert_eq!(settings.server.host, "127.0.0.1");
    }
}
