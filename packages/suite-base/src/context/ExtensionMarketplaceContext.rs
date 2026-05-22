```rust
use std::sync::{Arc, Mutex};

pub struct ExtensionInfo {
    pub name: String,
    pub version: String,
    // Add other fields as needed
}

#[derive(Debug)]
pub struct ExtensionMarketplaceDetail extends ExtensionInfo {
    sha256sum: Option<String>,
    foxe: Option<String>,
    time: Option<std::collections::BTreeMap<String, String>>,
}

pub type ExtensionMarketplace = Arc<Mutex<dyn ExtensionMarketplaceInterface>>;

trait ExtensionMarketplaceInterface {
    async fn get_available_extensions(&self) -> Vec<ExtensionMarketplaceDetail>;
    async fn get_markdown(&self, url: &str) -> String;
}

struct InMemoryExtensionMarketplace {
    extensions: Vec<ExtensionMarketplaceDetail>,
    markdown_cache: std::collections::HashMap<String, String>,
}

impl ExtensionMarketplaceInterface for InMemoryExtensionMarketplace {
    async fn get_available_extensions(&self) -> Vec<ExtensionMarketplaceDetail> {
        self.extensions.clone()
    }

    async fn get_markdown(&self, url: &str) -> String {
        if let Some(markdown) = self.markdown_cache.get(url) {
            return markdown.to_string();
        }
        // Fetch Markdown from URL and cache it
        unimplemented!() // Placeholder for actual implementation
    }
}

fn main() {
    let extension_marketplace = Arc::new(Mutex::new(InMemoryExtensionMarketplace {
        extensions: vec![ExtensionMarketplaceDetail {
            name: "Example Extension".to_string(),
            version: "1.0.0".to_string(),
            // Initialize other fields as needed
        }],
        markdown_cache: std::collections::HashMap::new(),
    }));

    // Use the extension_marketplace in your application
}
```