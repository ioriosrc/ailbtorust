```rust
use webpack::{Configuration as WebpackConfig, ResolveOptions};
use dotenv::dotenv;
use serde_json::Value;

fn load_env_vars() {
    dotenv().ok();
}

pub fn create_common_webpack_config(params: &WebpackConfigParams) -> WebpackConfig {
    let is_dev = params.mode == "development";
    let resolve: ResolveOptions = ResolveOptions {
        extensions: vec!["js", "ts", "tsx", "json"],
    };

    if !is_dev {
        // Stub out devtools installation for non-dev builds
        resolve.alias.insert("electron-devtools-installer".to_string(), false);
    }

    create_common_webpack_config(params, is_dev)
}

pub fn webpack_main_config(params: &WebpackConfigParams) -> WebpackConfig {
    load_env_vars();

    let is_serve = params.args.env.get("WEBPACK_SERVE").unwrap_or(&false).parse::<bool>().unwrap();
    let is_dev = params.mode == "development";

    let renderer_entry = if is_serve {
        format!("http://{}", params.args.host.unwrap_or("localhost"))
    } else {
        format!(
            "file://{}/renderer/index.html",
            path::join(env!("CARGO_MANIFEST_DIR"), "..", "renderer")
        )
    };

    WebpackConfig {
        context: Some(params.main_context),
        entry: Some(params.main_entrypoint.to_string()),
        target: "electron-main".to_string(),
        output: WebpackConfig::OutputOptions {
            path: params.output_path.join("main"),
            publicPath: "".to_string(),
        },
        plugins: vec![
            // ...(common.plugins.unwrap_or_default()), // Skip for simplicity
            webmanifest::DefinePlugin::new(vec![webmanifest::DefinePlugin::with_key(
                "MAIN_WINDOW_WEBPACK_ENTRY".to_string(),
                renderer_entry.to_string(),
            )]),
        ],
        resolve,
    }
}
```