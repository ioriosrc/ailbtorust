```rust
use std::collections::{HashMap, HashSet};

struct Logger;
type ExtensionInfo = /* Define your ExtensionInfo type here */;
type IExtensionLoader = /* Define your IExtensionLoader type here */;
type Namespace = String;
type ExtensionData = /* Define your ExtensionData type here */;
type ContributionPoints = /* Define your ContributionPoints type here */;
type LoadExtensionsResult = /* Define your LoadExtensionsResult type here */;

fn create_extension_registry_store(
  loaders: Vec<IExtensionLoader>,
  mock_message_converters: Option<Vec<RegisterMessageConverterArgs>>,
) -> Store {
  let org_cache_loader: Option<&IExtensionLoader> = loaders.iter().find(|loader| {
    loader.namespace == "org" && loader.type == "browser"
  });

  let mut store = Store::new();
  store.register_extension_registry_store(loaders, mock_message_converters);

  // Request an initial refresh on first mount
  store.refresh_all_extensions();

  store
}

pub struct ExtensionCatalogProvider {
  children: Box<dyn Fn() -> ReactElement>,
  loaders: Vec<IExtensionLoader>,
  mock_message_converters: Option<Vec<RegisterMessageConverterArgs>>,
}

impl ExtensionCatalogProvider {
  fn new(children: Box<dyn Fn() -> ReactElement>, loaders: Vec<IExtensionLoader>, mock_message_converters: Option<Vec<RegisterMessageConverterArgs>>) -> Self {
    Self { children, loaders, mock_message_converters }
  }
}

fn main() {
  // Your main code here
}
```