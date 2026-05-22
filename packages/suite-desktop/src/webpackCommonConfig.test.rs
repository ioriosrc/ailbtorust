```rust
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct WebpackConfigParams {
    package_json: PackageJson,
    main_context: String,
    main_entrypoint: String,
    renderer_context: String,
    renderer_entrypoint: String,
    quicklook_context: String,
    quicklook_entrypoint: String,
    preload_context: String,
    preload_entrypoint: String,
    outputPath: String,
    prod_source_map: String,
}

#[derive(Serialize, Deserialize)]
struct PackageJson {
    productName: String,
    name: String,
    version: String,
    description: String,
    productDescription: String,
    license: String,
    author: Author,
    homepage: String,
}

#[derive(Serialize, Deserialize)]
struct Author {
    name: String,
    email: String,
}

fn create_test_params(overrides: Option<WebpackConfigParams>) -> WebpackConfigParams {
    let mut params = WebpackConfigParams {
        package_json: PackageJson {
            productName: "Test Product".to_string(),
            name: "test-product".to_string(),
            version: "1.0.0".to_string(),
            description: "Test Description".to_string(),
            productDescription: "Test Product Description".to_string(),
            license: "MPL-2.0".to_string(),
            author: Author {
                name: "Test Author".to_string(),
                email: "test@example.com".to_string(),
            },
            homepage: "https://example.com".to_string(),
        },
        main_context: "/test/main".to_string(),
        main_entrypoint: "./index.ts",
        renderer_context: "/test/renderer".to_string(),
        renderer_entrypoint: "./index.tsx",
        quicklook_context: "/test/quicklook".to_string(),
        quicklook_entrypoint: "./index.tsx",
        preload_context: "/test/preload".to_string(),
        preload_entrypoint: "./index.tsx",
        outputPath: "/test/output".to_string(),
        prod_source_map: "source-map".to_string(),
    };

    if let Some(overrides) = overrides {
        params.merge(&overrides);
    }

    params
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize, Deserialize)]
    struct ExpectedConfigParams {
        devtool: String,
        resolve_extensions: Vec<String>,
        module_rules: Vec<ModuleRule>,
        optimization_minimizer: Vec<OptimizationMinimizer>,
        plugins: Vec<Plugin>,
    }

    #[derive(Serialize, Deserialize)]
    struct ModuleRule {
        use: ModuleUse,
    }

    #[derive(Serialize, Deserialize)]
    struct ModuleUse {
        loader: String,
    }

    #[derive(Serialize, Deserialize)]
    struct Plugin {
        plugin_name: String,
    }

    fn expected_config(is_dev: bool) -> ExpectedConfigParams {
        let mut plugins = vec![
            Plugin {
                plugin_name: "DefinePlugin".to_string(),
            },
            Plugin {
                plugin_name: "EsbuildPlugin".to_string(),
            },
            Plugin {
                plugin_name: "ForkTsCheckerWebpackPlugin".to_string(),
            },
        ];

        if is_dev {
            return ExpectedConfigParams {
                devtool: String::from("eval-cheap-module-source-map"),
                resolve_extensions: vec![".tsx"],
                module_rules: vec![ModuleRule { use: ModuleUse { loader: "ts-loader".to_string() } }],
                optimization_minimizer: vec![OptimizationMinimizer {
                    plugin_name: "EsbuildPlugin".to_string(),
                }],
                plugins,
            };
        }

        return ExpectedConfigParams {
            devtool: String::from("source-map"),
            resolve_extensions: vec![".tsx"],
            module_rules: vec![ModuleRule { use: ModuleUse { loader: "ts-loader".to_string() } }],
            optimization_minimizer: vec![
                OptimizationMinimizer {
                    plugin_name: "EsbuildPlugin".to_string(),
                    options: Some(OptimizationOptions {
                        remove_available_modules: true,
                    }),
                },
            ],
            plugins,
        };
    }

    #[test]
    fn test_create_common_webpack_config_dev() {
        let params = create_test_params(None);
        let config = create_common_webpack_config(&params, &expected_config(false));
        assert_eq!(config.devtool, String::from("eval-cheap-module-source-map"));
        assert_eq!(config.resolve_extensions, vec![".tsx"]);
        assert_eq!(
            config.module_rules[0].use.loader,
            String::from("ts-loader")
        );
    }

    #[test]
    fn test_create_common_webpack_config_prod() {
        let params = create_test_params(None);
        let config = create_common_webpack_config(&params, &expected_config(true));
        assert_eq!(config.devtool, String::from("source-map"));
        assert_eq!(config.resolve_extensions, vec![".tsx"]);
        assert_eq!(
            config.module_rules[0].use.loader,
            String::from("ts-loader")
        );
    }
}
```