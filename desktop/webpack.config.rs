```rust
use std::path;

use webpack_config_params::{WebpackConfigParams, webpack_dev_server_config};
use webpack_main_config::webpack_main_config;
use webpack_preload_config::webpack_preload_config;
use webpack_renderer_config::webpack_renderer_config;
use webpack_quicklook_config::webpack_quicklook_config;

fn main() {
    let package_json = include_str!("../package.json");
    let params: WebpackConfigParams = WebpackConfigParams {
        package_json,
        outputPath: path::PathBuf::from(".webpack"),
        prodSourceMap: "source-map",
        rendererContext: path::PathBuf::from("renderer"),
        rendererEntrypoint: "./index.ts",
        mainContext: path::PathBuf::from("main"),
        mainEntrypoint: "./index.ts",
        quicklookContext: path::PathBuf::from("quicklook"),
        quicklookEntrypoint: "./index.ts",
        preloadContext: path::PathBuf::from("preload"),
        preloadEntrypoint: "./index.ts",
    };

    let webpack_dev_server_config = webpack_dev_server_config(&params);
    let webpack_main_config = webpack_main_config(&params);
    let webpack_preload_config = webpack_preload_config(&params);
    let webpack_renderer_config = webpack_renderer_config(&params);
    let webpack_quicklook_config = webpack_quicklook_config(&params);

    println!("{:?}", [
        webpack_dev_server_config,
        webpack_main_config,
        webpack_preload_config,
        webpack_renderer_config,
        webpack_quicklook_config,
    ]);
}
```