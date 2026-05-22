```rust
use anyhow::Error;
use async_std::fs::{read_to_string, File};
use serde_json::{self};

struct MockExtensionLoader;

impl IExtensionLoader for MockExtensionLoader {
    type Namespace = String;
    type ExtensionInfo = ExtensionInfo;

    fn get_extension(&self, id: &str) -> Result<Self::ExtensionInfo, Error> {
        let path = format!("{}/{}-{}.json", std::env!("CARGO_TARGET_DIR"), self.namespace(), id);
        if !File::exists(&path).await? {
            return Err(anyhow!("Extension not found"));
        }
        Ok(serde_json::from_str(&read_to_string(path).await?)?)
    }

    fn get_extensions(&self) -> Result<Vec<Self::ExtensionInfo>, Error> {
        let path = format!("{}/{}", std::env!("CARGO_TARGET_DIR"), self.namespace());
        if !File::exists(&path).await? {
            return Ok(vec![]);
        }
        Ok(serde_json::from_str(&read_to_string(path).await?)?)
    }

    async fn load_extension(&self, id: &str) -> Result<ExtensionInfo, Error> {
        self.get_extension(id)
    }

    async fn install_extension(&self, _foxe_file_data: String) -> Result<(), Error> {
        Err(anyhow!("Not implemented"))
    }

    async fn uninstall_extension(&self, _id: &str) -> Result<(), Error> {
        Ok(())
    }
}

struct MockExtensionMarketplace;

impl ExtensionMarketplace for MockExtensionMarketplace {
    type AvailableExtension = ExtensionInfo;
    type Markdown = String;

    async fn get_available_extensions(&self) -> Result<Vec<Self::AvailableExtension>, Error> {
        let path = format!("{}/{}", std::env!("CARGO_TARGET_DIR"), "extensionmarketplace");
        if !File::exists(&path).await? {
            return Ok(vec![]);
        }
        Ok(serde_json::from_str(&read_to_string(path).await?)?)
    }

    async fn get_markdown(&self, url: String) -> Result<Self::Markdown, Error> {
        let path = format!("{}/{}", std::env!("CARGO_TARGET_DIR"), "extensionmarketplace/markdown", url);
        if !File::exists(&path).await? {
            return Err(anyhow!("Markdown not found"));
        }
        Ok(serde_json::from_str(&read_to_string(path).await?)?)
    }
}

struct ExtensionInfo {
    id: String,
    name: String,
    qualified_name: String,
    display_name: String,
    description: String,
    publisher: String,
    homepage: String,
    license: String,
    version: String,
    keywords: Vec<String>,
    namespace: String,
    readme: String,
    changelog: String,
}

fn make_mock_app_configuration() -> AppConfiguration {
    AppConfiguration {
        // Initialize the configuration here
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = make_mock_app_configuration();

    let market_place = MockExtensionMarketplace;

    let extensions_settings: ExtensionsSettings = ExtensionsSettings {
        app_configuration: config,
        extension_catalog_provider: ExtensionCatalogProvider::new(vec![MockExtensionLoader]),
        extension_marketplace_context: ExtensionMarketplaceContext::new(market_place),
    };

    Ok(())
}
```