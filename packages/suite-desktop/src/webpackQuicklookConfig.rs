```rust
use webpack::{Configuration, ProvidePlugin};
use webpack_dev_server::WebpackDevServerOptions;

fn main() -> webpack::Result<Configuration> {
    let mut config = Configuration {
        name: "quicklook",
        context: Some("path/to/your/context"),
        entry: Some("path/to/your/entrypoint"),
        target: "web",
        devtool: Some(webpack::DevtoolType::EvalcheapModuleSourceMap),
        output: Some(Configuration::Output {
            publicPath: Some(String::from("/quicklook/")),
            path: Some(path::PathBuf::from("path/to/your/output/directory")),
        }),
        module: Some(Configuration::Module {
            rules: vec![
                Configuration::Rule::new()
                    .test(r"\.png$|\.wasm$")
                    .type(webpack::ModuleType::Asset::Inline),
                Configuration::Rule::new()
                    .test(r"\.tsx?")
                    .exclude(Some(PathBuf::from("node_modules")))
                    .use(vec![
                        Box::new(webpack::loader::FileLoader::default()),
                        Box::new(webpack::loader::TsLoader {
                            transpile_only: true,
                            only_compile_bundled_files: Some(true),
                            project_references: Some(true),
                            get_custom_transformers: Box::new(||
                                vec![Box::new(webpack::loader::ReactRefreshTypescript())]),
                            }),
                        ]),
                    ]),
            ],
        }),
        optimization: Some(Configuration::Optimization {
            remove_available_modules: true,
            minimizer: vec![
                Box::new(webpack::minimizer::BabelMinimizer {
                    presets: Some(vec!["@babel/preset-env".to_string()]),
                }),
            ],
        }),
        plugins: vec![
            Box::new(webpack::plugin::ProvidePlugin({
                "Buffer": ["buffer", "Buffer"],
            })),
            if std::env::var("WEBPACK_SERVE").is_ok() {
                Box::new(webpack::dev_server::WebpackDevServerOptions {
                    hot: true,
                }),
            } else {
                Box::new(webpack::plugin::ForkTsCheckerWebpackPlugin({
                    typescript: Some(Configuration::TypescriptConfig {
                        config_overwrite: Some(
                            serde_json::to_string_pretty(&serde_yaml::from_str::<HashMap<String, Value>>(
                                r#"
{
  "compilerOptions": {
    "noUnusedLocals": !process.env.WEBPACK_SERVE,
    "noUnusedParameters": !process.env.WEBPACK_SERVE
  }
}
"#).unwrap(),
                            )
                        ),
                    }),
                })),
            },
        ],
        resolve: Some(Configuration::Resolve {
            extensions: vec![".js", ".ts", ".tsx", ".json"],
            fallback: {
                "path": require.resolve("path-browserify"),
                "stream": false,
                "crypto": false,
                "fs": false,
            },
        }),
    };

    Ok(config)
}
```