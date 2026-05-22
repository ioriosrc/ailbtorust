```rust
mod browser_storage_keys {
    // Define constants based on the TypeScript/React code
    const KEY_WORKSPACE_PREFIX: &str = "";
    const LOCAL_STORAGE_STUDIO_LAYOUT_KEY: &str = "studio.layout";
    const LOCAL_STORAGE_PROFILE_DATA: &str = "studio.profile-data";
    const LOCAL_STORAGE_APP_CONFIGURATION: &str = "studio.app-configuration.";
    const SESSION_STORAGE_LOGS_SETTINGS: &str = "blick.logs-settings";
    const SESSION_STORAGE_LICHTBLICK_WORKSPACE: &str = "fox.workspace";
    const SESSION_STORAGE_I18N_LANGUAGE: &str = "i18nextLng";

    // Function to simulate browser storage operations (not implemented in Rust)
    async fn test_storage_keys() {
        // Test that all local storage keys are properly defined
        assert_eq!(KEY_WORKSPACE_PREFIX, "");
        assert_eq!(LOCAL_STORAGE_STUDIO_LAYOUT_KEY, "studio.layout");
        assert_eq!(LOCAL_STORAGE_PROFILE_DATA, "studio.profile-data");
        assert_eq!(
            LOCAL_STORAGE_APP_CONFIGURATION,
            "studio.app-configuration.",
        );
        assert_eq!(
            SESSION_STORAGE_LOGS_SETTINGS,
            "blick.logs-settings",
        );
        assert_eq!(
            SESSION_STORAGE_LICHTBLICK_WORKSPACE,
            "fox.workspace",
        );
        assert_eq!(
            SESSION_STORAGE_I18N_LANGUAGE,
            "i18nextLng",
        );

        // Test that all session storage keys are properly defined
        assert_eq!(KEY_WORKSPACE_PREFIX, "");
        assert_eq!(LOCAL_STORAGE_STUDIO_LAYOUT_KEY, "studio.layout");
        assert_eq!(LOCAL_STORAGE_PROFILE_DATA, "studio.profile-data");
        assert_eq!(
            LOCAL_STORAGE_APP_CONFIGURATION,
            "studio.app-configuration.",
        );
        assert_eq!(
            SESSION_STORAGE_LOGS_SETTINGS,
            "blick.logs-settings",
        );
        assert_eq!(
            SESSION_STORAGE_LICHTBLICK_WORKSPACE,
            "fox.workspace",
        );
        assert_eq!(
            SESSION_STORAGE_I18N_LANGUAGE,
            "i18nextLng",
        );
    }
}
```