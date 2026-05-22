```rust
use webpack::{config, utils::Path};

#[derive(Debug)]
struct WebpackConfigParams {
    package_json: serde_json::Value,
    main_context: Path,
    main_entrypoint: Path,
    renderer_context: Path,
    renderer Entrypoint: Path,
    quicklook_context: Path,
    quicklook_entrypoint: Path,
    preload_context: Path,
    preload_entrypoint: Path,
    outputPath: Path,
    prod_source_map: String,
}

fn create_test_params(overrides: Option<serde_json::Value>) -> WebpackConfigParams {
    let mut params = WebpackConfigParams {
        package_json: serde_json::json!({
            "productName": "Test Product",
            "name": "test-product",
            "version": "1.0.0",
            "description": "Test Description",
            "productDescription": "Test Product Description",
            "license": "MPL-2.0",
            "author": { "name": "Test Author", "email": "test@example.com" },
            "homepage": "https://example.com",
        }),
        main_context: Path::new("/test/main"),
        main_entrypoint: Path::new("./index.ts"),
        renderer_context: Path::new("/test/renderer"),
        renderer_entrypoint: Path::new("./index.tsx"),
        quicklook_context: Path::new("/test/quicklook"),
        quicklook_entrypoint: Path::new("./index.ts"),
        preload_context: Path::new("/test/preload"),
        preload_entrypoint: Path::new("./index.ts"),
        outputPath: Path::new("/test/output"),
        prod_source_map: "source-map".to_string(),
    };

    if let Some(overrides) = overrides {
        params.package_json = overrides;
    }

    params
}

fn main() {
    use std::fs::File;
    use std::io::Write;

    let json = serde_json::to_string_pretty(&create_test_params(None)).unwrap();
    File::create("webpack_config.json").unwrap().write_all(json.as_bytes()).unwrap();
}
```