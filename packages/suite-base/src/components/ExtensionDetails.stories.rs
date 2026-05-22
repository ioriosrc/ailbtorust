```rust
use std::rc::Rc;

use super::{
  ExtensionDetails,
  AppConfigurationContext,
  ExtensionMarketplaceContext,
  ExtensionCatalogProvider,
  IExtensionLoader,
  makeMockAppConfiguration,
};

#[derive(Default, Clone)]
pub struct MockExtensionLoader {
    type: String,
    namespace: String,
    get_extension: Box<dyn Fn() -> Result<(), ()>>,
    get_extensions: Box<dyn Fn() -> Vec<crate::types::ExtensionDetail> + 'static>,
    load_extension: Box<dyn Fn(String) -> crate::types::LoadedExtensionResult>,
    install_extension: Box<dyn Fn(crate::types::InstallOptions<'_>) -> Result<(), ()>>,
    uninstall_extension: Box<dyn Fn(String) -> ()>,
}

impl IExtensionLoader for MockExtensionLoader {
    fn type(&self) -> &str {
        &self.type
    }

    fn namespace(&self) -> &str {
        &self.namespace
    }

    async fn get_extension(&self) -> Result<(), ()> {
        (self.get_extension)();
    }

    async fn get_extensions(&self) -> Vec<crate::types::ExtensionDetail> {
        (self.get_extensions)();
    }

    async fn load_extension(&self, id: String) -> crate::types::LoadedExtensionResult {
        (self.load_extension)(id)
    }

    async fn install_extension(&self, options: crate::types::InstallOptions<'_>) -> Result<(), ()> {
        (self.install_extension)(options);
    }

    async fn uninstall_extension(&self, id: String) {
        (self.uninstall_extension)(id);
    }
}

#[derive(Default)]
pub struct MockExtensionMarketplace {
    get_available_extensions: Box<dyn Fn() -> Vec<crate::types::ExtensionDetail> + 'static>,
    get_markdown: Box<dyn Fn(String) -> String + 'static>,
}

impl crate::data::ExtensionMarketplace for MockExtensionMarketplace {
    async fn get_available_extensions(&self) -> Vec<crate::types::ExtensionDetail> {
        (self.get_available_extensions)();
    }

    async fn get_markdown(&self, url: String) -> String {
        (self.get_markdown)(url);
    }
}

pub const extension: crate::data::ExtensionMarketplaceDetail = crate::data::ExtensionMarketplaceDetail {
    id: "publisher.storyextension",
    name: "Extension Name",
    description: "Extension sample description",
    qualified_name: "Qualified Extension Name",
    publisher: "Publisher",
    homepage: "https://github.com/lichtblick-suite",
    license: "MIT",
    version: "1.2.10",
    keywords: vec!["storybook", "testing"],
    displayName: "Display Extension Name",
    time: {
        modified: "2021-05-19T21:37:40.166Z",
        created: "2012-04-17T00:38:04.350Z",
        "0.0.2": "2012-04-17T00:38:05.679Z",
        "2.1.0": "2021-05-19T21:37:38.037Z",
    },
};

pub const Details: fn(Rc<AppConfigurationContext>) -> Box<dyn Fn() -> JSX.Element> = |config| {
    use crate::components::ExtensionDetails as ExtensionDetailsComponent;
    Box::new(move || {
        ExtensionDetailsComponent {
            config: Rc::clone(&config),
            extension: Rc::from(extension.clone()),
            onClose: |_|
                panic!("MockDetails onClose not implemented"),
            installed: false,
        }
    })
};
```