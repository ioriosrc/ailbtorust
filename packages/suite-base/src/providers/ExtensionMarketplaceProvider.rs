```rust
use std::error::Error;
use reqwest::{Client, Response};
use serde_json::{self, Value};

async fn fetch_marketplace_url() -> Result<Vec<Value>, Box<dyn Error>> {
    let client = Client::new();
    let response = client.get("https://raw.githubusercontent.com/foxglove/studio-extension-marketplace/main/extensions.json").await?;
    Ok(response.json().await?)
}

#[derive(Debug)]
struct ExtensionMarketplaceDetail {
    name: String,
    description: String,
    version: String,
    // Add other fields as needed
}

pub struct ExtensionMarketplaceProvider {
    available_extensions: Vec<ExtensionMarketplaceDetail>,
    markdown_content: std::sync::Arc<String>,
}

impl ExtensionMarketplaceProvider {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let available_extensions = fetch_marketplace_url().await?;
        let markdown_content = format!(
            "## Available Extensions\n\n{:?}",
            &available_extensions
        );
        Ok(Self {
            available_extensions,
            markdown_content: Arc::new(markdown_content),
        })
    }

    pub fn get_available_extensions(&self) -> &[ExtensionMarketplaceDetail] {
        &self.available_extensions
    }

    pub async fn fetch_markdown(&mut self, url: &str) -> Result<(), Box<dyn Error>> {
        let response = reqwest::get(url).await?;
        self.markdown_content = Arc::new(response.text().await?);
        Ok(())
    }
}

#[derive(Debug)]
pub struct ExtensionMarketplaceContext {
    provider: ExtensionMarketplaceProvider,
}

impl ExtensionMarketplaceContext {
    pub fn new(provider: ExtensionMarketplaceProvider) -> Self {
        Self { provider }
    }

    pub fn provide(&self, children: std::rc::Rc<crate::prelude::Element>) -> crate::prelude::Element {
        <ExtensionMarketplaceContextProvider as crate::prelude::Provider>::provide(self, children)
    }
}

struct ExtensionMarketplaceContextProvider;

impl crate::prelude::Provider for ExtensionMarketplaceContextProvider {
    type Item = ExtensionMarketplaceContext;

    fn provide(&self, children: std::rc::Rc<crate::prelude::Element>) -> crate::prelude::Element {
        let context = ExtensionMarketplaceContext {
            provider: self.provider.clone(),
        };
        <ExtensionMarketplaceContextProvider as crate::prelude::Provider>::provide(self, children)
    }
}
```