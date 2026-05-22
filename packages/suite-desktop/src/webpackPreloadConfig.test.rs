```rust
use serde_json::{Deserialize, Serialize};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

#[derive(Serialize, Deserialize, Debug)]
struct WebpackArgv {
    mode: String,
    env: std::collections::HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct WebpackConfigParams {
    package_json: WebpackPackageJson,
    main_context: String,
    main_entrypoint: String,
    renderer_context: String,
    renderer_entropoint: String,
    quicklook_context: String,
    quicklook_entropoint: String,
    preload_context: String,
    preload_entropoint: String,
    outputPath: String,
    prod_source_map: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct WebpackPackageJson {
    productName: String,
    name: String,
    version: String,
    description: String,
    productDescription: String,
    license: String,
    author: Author,
    homepage: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Author {
    name: String,
    email: String,
}

fn create_test_params(overrides: Option<WebpackConfigParams>) -> WebpackConfigParams {
    let mut params = WebpackConfigParams {
        package_json: WebpackPackageJson {
            productName: "Test Product".to_string(),
            name: "test-product".to_string(),
            version: "1.0.0".to_string(),
            description: "Test Description".to_string(),
            product_description: "Test Product Description".to_string(),
            license: "MPL-2.0".to_string(),
            author: Author {
                name: "Test Author".to_string(),
                email: "test@example.com".to_string(),
            },
            homepage: "https://example.com".to_string(),
        },
        main_context: "/test/main".to_string(),
        main_entrypoint: "./index.ts".to_string(),
        renderer_context: "/test/renderer".to_string(),
        renderer_entropoint: "./index.tsx".to_string(),
        quicklook_context: "/test/quicklook".to_string(),
        quicklook_entropoint: "./index.ts",
        preload_context: "/test/preload".to_string(),
        preload_entropoint: "./index.ts",
        outputPath: "/test/output".to_string(),
        prod_source_map: "source-map".to_string(),
    };

    if let Some(overrides) = overrides {
        params.update(&overrides);
    }

    return params;
}

mod webpack_common_config {
    use serde_json::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct CommonWebpackConfigParams {
        target: String,
        devtool: String,
        plugins: Vec<Plugin>,
    }

    #[derive(Serialize, Deserialize)]
    struct Plugin {
        plugin_name: String,
    }

    pub fn create_common_webpack_config(params: CommonWebpackConfigParams, is_dev: bool) -> CommonWebpackConfigParams {
        let mut config = params;
        config.devtool = if is_dev {
            "eval-cheap-module-source-map".to_string()
        } else {
            "source-map".to_string()
        };
        config.plugins.push(Plugin { plugin_name: "MockPlugin".to_string() });
        return config;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_params_json(overrides: Option<WebpackConfigParams>) -> String {
        serde_json::to_string(&create_test_params(overrides)).unwrap()
    }

    #[test]
    fn webpack_preload_config_development_mode() {
        let params = WebpackConfigParams {
            package_json: WebpackPackageJson {
                productName: "Test Product".to_string(),
                name: "test-product".to_string(),
                version: "1.0.0".to_string(),
                description: "Test Description".to_string(),
                product_description: "Test Product Description".to_string(),
                license: "MPL-2.0".to_string(),
                author: Author {
                    name: "Test Author".to_string(),
                    email: "test@example.com".to_string(),
                },
                homepage: "https://example.com".to_string(),
            },
            main_context: "/test/main".to_string(),
            main_entrypoint: "./index.ts".to_string(),
            renderer_context: "/test/renderer".to_string(),
            renderer_entropoint: "./index.tsx".to_string(),
            quicklook_context: "/test/quicklook".to_string(),
            quicklook_entropoint: "./index.ts",
            preload_context: "/test/preload".to_string(),
            preload_entropoint: "./index.ts",
            outputPath: "/test/output".to_string(),
            prod_source_map: "source-map".to_string(),
        };

        let argv: WebpackArgv = {
            mode: "development".to_string(),
            env: std::collections::HashMap::new(),
        };

        let config = webpack_preload_config(&params, &argv);
        assert_eq!(config.target, "electron-preload");
        assert_eq!(config.devtool, "eval-cheap-module-source-map");
        assert_eq!(config.context, "/test/preload");
        assert_eq!(config.entry, "./index.ts");
        assert_eq!(config.output?.filename, "preload.js");
        assert_eq!(config.output?.path, path.join("/test/output", "main"));
    }

    #[test]
    fn webpack_preload_config_production_mode() {
        let params = WebpackConfigParams {
            package_json: WebpackPackageJson {
                productName: "Test Product".to_string(),
                name: "test-product".to_string(),
                version: "1.0.0".to_string(),
                description: "Test Description".to_string(),
                product_description: "Test Product Description".to_string(),
                license: "MPL-2.0".to_string(),
                author: Author {
                    name: "Test Author".to_string(),
                    email: "test@example.com".to_string(),
                },
                homepage: "https://example.com".to_string(),
            },
            main_context: "/test/main".to_string(),
            main_entrypoint: "./index.ts".to_string(),
            renderer_context: "/test/renderer".to_string(),
            renderer_entropoint: "./index.tsx".to_string(),
            quicklook_context: "/test/quicklook".to_string(),
            quicklook_entropoint: "./index.ts",
            preload_context: "/test/preload".to_string(),
            preload_entropoint: "./index.ts",
            outputPath: "/test/output".to_string(),
            prod_source_map: "source-map".to_string(),
        };

        let argv: WebpackArgv = {
            mode: "production".to_string(),
            env: std::collections::HashMap::new(),
        };

        let config = webpack_preload_config(&params, &argv);
        assert_eq!(config.target, "electron-preload");
        assert_eq!(config.devtool, "source-map");
        assert_eq!(config.output?.filename, "preload.js");
        assert_eq!(config.output?.public_path, "");
        assert_eq!(config.output?.path, path.join("/test/output", "main"));
    }

    #[test]
    fn webpack_preload_config_common_config_integration() {
        let params = WebpackConfigParams {
            package_json: WebpackPackageJson {
                productName: "Test Product".to_string(),
                name: "test-product".to_string(),
                version: "1.0.0".to_string(),
                description: "Test Description".to_string(),
                product_description: "Test Product Description".to_string(),
                license: "MPL-2.0".to_string(),
                author: Author {
                    name: "Test Author".to_string(),
                    email: "test@example.com".to_string(),
                },
                homepage: "https://example.com".to_string(),
            },
            main_context: "/test/main".to_string(),
            main_entrypoint: "./index.ts".to_string(),
            renderer_context: "/test/renderer".to_string(),
            renderer_entropoint: "./index.tsx".to_string(),
            quicklook_context: "/test/quicklook".to_string(),
            quicklook_entropoint: "./index.ts",
            preload_context: "/test/preload".to_string(),
            preload_entropoint: "./index.ts",
            outputPath: "/test/output".to_string(),
            prod_source_map: "source-map".to_string(),
        };

        let argv: WebpackArgv = {
            mode: "development".to_string(),
            env: std::collections::HashMap::new(),
        };

        let config = webpack_preload_config(&params, &argv);

        assert_eq!(config.plugins.len(), 1);
        assert_eq!(config.devtool, "eval-cheap-module-source-map");
    }
}
```