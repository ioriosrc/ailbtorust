```rust
use webpack::{Configuration, DevServerOptions};
use webpack_dev_server::DevMiddlewareOptions;

pub fn webpack_dev_server_config(params: &WebpackConfigParams) -> Configuration {
    let is_release = match params.args.mode.as_str() {
        "production" => true,
        _ => false,
    };

    // The appdata directory is derived from the product name. To have a separate directory
    // for our production and development builds we change the product name when using dev or serve.
    let productName = if is_release {
        params.package_json.product_name.clone()
    } else {
        format!("{} Dev", params.package_json.product_name.clone())
    };

    Configuration {
        entry: vec![],
        output: OutputOptions {
            path: params.output_path.to_string(),
            public_path: "".to_string(),
        },
        dev_server: DevServerOptions {
            static_options: StaticOptions {
                directory: params.output_path.to_string(),
            },
            hot: true,
            allowed_hosts: Some(vec!["*".to_string()]),
            client: ClientOptions {
                overlay: Some(ClientOverlayOptions {
                    runtime_errors: false,
                }),
            },
            dev_middleware: DevMiddlewareOptions {
                writeTo_disk: |filePath| {
                    // Electron needs to open the main thread source and preload source from disk
                    // avoid writing the hot-update js and json files
                    // allow writing package.json at root -> needed for electron to find entrypoint
                    filePath.starts_with(&"webpack[\\/]((main|extensions)[\\/](?!.*hot-update)|package\\.json)/")
                },
            },
        },
        plugins: vec![
            Box::new(CleanWebpackPlugin {}),
            Box::new(HtmlWebpackPlugin {
                filename: "package.json".to_string(),
                template_content: format!(
                    r#"{{"main": "{{ main }}","name":"{}","productName":"{}","version":"{}","description":"{}","productDescription":"{}","license":"{}","author":"{}"}}"#,
                    params.package_json.name.clone(),
                    productName,
                    params.package_json.version.clone(),
                    params.package_json.description.clone(),
                    params.package_json.product_description.clone(),
                    params.package_json.license.clone(),
                    params.package_json.author.clone(),
                ),
            }),
        ],
    }
}
```