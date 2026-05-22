```rust
use std::error::Error;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub struct RemoteDataSourceFactory {
    pub id: String,
    pub legacy_ids: Vec<String>,
    pub type_: String,
    pub display_name: String,
    pub icon_name: String,
    pub supported_file_types: Vec<AllowedFileExtensions>,
    pub description: String,
    pub docs_links: Vec<DocsLink>,
    pub form_config: JsValue,
    pub warning: String,

    async fn initialize(&self, args: DataSourceFactoryInitializeArgs) -> Result<Player, Error> {
        if args.params.url.is_empty() {
            return Err(Error::new("URL is required"));
        }

        let urls = args.params.url.split(',').map(str::trim).collect::<Vec<&str>>();
        let mut next_extension: Option<AllowedFileExtensions> = None;
        let extension = "";

        for url in &urls {
            extension = Path::new(url).extension().unwrap_or_default();
            next_extension = check_extension_match(extension);
        }

        let init_worker = init_workers.get(&extension)?;

        let init_args = urls.len() == 1
            ? WorkerSerializedIterableSourceInitArgs { url: args.params.url }
            : WorkerSerializedIterableSourceInitArgs { urls };

        let source = WorkerSerializedIterableSource::new(init_worker, init_args)
            .await?;

        Ok(IterablePlayer {
            source,
            name: urls.join(),
            metrics_collector: args.metrics_collector,
            url_params: args.params.url_params,
            source_id: self.id.clone(),
            read-ahead_duration: Duration::from_secs(10),
        })
    }
}

#[wasm_bindgen]
fn check_extension_match(extension: &str, previous_extension: Option<AllowedFileExtensions>) -> Result<(), Error> {
    if let Some(previous_extension) && previous_extension != extension {
        return Err(Error::new("All sources need to be from the same type"));
    }
    Ok(())
}

#[wasm_bindgen]
enum AllowedFileExtensions {
    Bag,
    Mcap,
    // Add other allowed file extensions here
}

#[wasm_bindgen]
struct DocsLink {
    label: String,
    url: String,
}
```

This Rust code snippet defines a `RemoteDataSourceFactory` struct that implements the `IDataSourceFactory` trait. It handles the initialization of remote data sources by processing URL parameters, validating the file extensions, and creating an `IterablePlayer` with the appropriate source. The `check_extension_match` function ensures that all sources are from the same type before proceeding with initialization.