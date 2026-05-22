```rust
use std::env;

async fn main() {
    let app_configuration = NativeStorageAppConfiguration::initialize(
        env::var("STORAGE_BRIDGE").unwrap().parse::<storage_bridge::Bridge>().unwrap(),
        AppSetting::SHOW_DEBUG_PANELS, // Assuming this is a constant or defined in your Rust project
    ).await.unwrap();

    renderer_main(app_configuration);
}
```

In this Rust code, we have replaced the TypeScript/React code with its equivalent Rust syntax. The `NativeStorageAppConfiguration` and `rendererMain` functions are assumed to be defined in separate modules like `suite-base`, `suite-desktop/src/common/types`, and `suite-desktop/src/renderer/index.js`, respectively. You would need to import these modules and define the constants or variables that correspond to the TypeScript/React code.