```rust
use serde::Deserialize;

#[derive(Deserialize)]
pub struct WebpackConfigParams {
    package_json: PackageJson,
}

#[derive(Deserialize)]
struct PackageJson {
    productName: String,
    version: String,
    homepage: String,
    suite_version: String,
}

fn create_common_webpack_config(params: WebpackConfigParams, is_dev: bool) -> Option<Configuration> {
    Some(Configuration {
        devtool: if is_dev {
            "eval-cheap-module-source-map"
        } else {
            params.prod_source_map
        },

        module: Module {
            rules: vec![Rule {
                test: r".tsx?$/,
                exclude: vec!["node_modules/"],
                use: vec![Loader::new("ts-loader")
                    .options(ConfigurableOptions::new()
                        .transpile_only(true)
                        // https://github.com/TypeStrong/ts-loader#onlycompilebundledfiles
                        // avoid looking at files which are not part of the bundle
                        .only_compile_bundled_files(true)
                        .project_references(true))],
            }],
        },

        optimization: Optimization {
            remove_available_modules: true,
            minimizer: vec![Minimizer::new(EsbuildPlugin {
                target: "es2022",
                minify: true,
            })],
        },

        plugins: vec![
            Plugin::new(DefinePlugin({
                // Should match webpack-defines.d.ts
                ReactNull: "null",
                LICHTBLICK_PRODUCT_NAME: format!("\"{}\"", params.package_json.productName),
                LICHTBLICK_PRODUCT_VERSION: format!("\"{}\"", params.package_json.version),
                LICHTBLICK_PRODUCT_HOMEPAGE: format!("\"{}\"", params.package_json.homepage),
                LICHTBLICK_SUITE_VERSION: format!("\"{}\"", params.package_json.suite_version),
                API_URL: env::var("API_URL")
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| None),
                DEV_WORKSPACE: env::var("DEV_WORKSPACE")
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| None),
            })),
            Plugin::new(ForkTsCheckerWebpackPlugin()),
        ],

        resolve: Resolve {
            extensions: vec![".js", ".ts", ".tsx", ".json"],
        },
    })
}

fn main() {
    // Implement your application logic here
}
```

Note that this Rust code is a simplified representation of the original TypeScript/React code and may not be directly translatable to Rust due to differences in syntax, modules, and libraries. The actual translation would require knowledge of Rust's ecosystem, specifically how it handles dependencies, Webpack configuration, and environment variables.