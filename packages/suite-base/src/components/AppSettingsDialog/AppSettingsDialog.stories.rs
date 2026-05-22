```rust
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use storybook::prelude::*;
use storybook_testing_library::{screen, user_event};
use mockall::mock;

mod your_module {
    pub use super::*;

    pub struct MockExtensionLoader {
        installed_extensions: Vec<ExtensionInfo>,
        marketplace_extensions: Vec<ExtensionInfo>,
    }

    #[async_trait]
    impl IExtensionLoader for MockExtensionLoader {
        async fn type(&self) -> &'static str {
            "browser"
        }

        async fn namespace(&self) -> &str {
            "local"
        }

        async fn get_extension(&self, _id: &str) -> Result<ExtensionInfo, Box<dyn std::error::Error>> {
            Ok(self.installed_extensions.first().cloned().unwrap_or_default())
        }

        async fn get_extensions(&self) -> Result<Vec<ExtensionInfo>, Box<dyn std::error::Error>> {
            Ok(self.installed_extensions.clone())
        }

        async fn load_extension(&self, _foxe_file_data: &str) -> Result<(), Box<dyn std::error::Error>> {
            Err(Box::new(Error("MockExtensionLoader cannot install extensions")))
        }

        async fn uninstall_extension(&self, _id: &str) -> Result<(), Box<dyn std::error::Error>> {
            Ok(())
        }
    }

    pub struct MockExtensionMarketplace {
        available_extensions: Vec<ExtensionInfo>,
    }

    impl ExtensionMarketplace for MockExtensionMarketplace {
        async fn get_available_extensions(&self) -> Result<Vec<ExtensionInfo>, Box<dyn std::error::Error>> {
            Ok(self.available_extensions.clone())
        }

        async fn get_markdown(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
            Ok(format!(
                "# Markdown\nMock markdown rendering for URL [{}]({})\n",
                url, url
            ))
        }
    }
}

mod your_module_ui {
    pub use super::*;

    #[derive(Props)]
    struct AppSettingsDialogProps {
        open: bool,
        active_tab: String,
    }

    pub fn AppSettingsDialog(_props: &AppSettingsDialogProps) -> JSXElement {
        // Your implementation here
    }
}
```