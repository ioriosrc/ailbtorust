```rust
use std::env;

use webpack::{Configuration, ModuleRule, WebpackError};

pub fn make_config(
    _: &std::any::Any,
    argv: webpack_argv::WebpackArgv,
    options: MakeConfigOptions,
) -> Result<Configuration, WebpackError> {
    let is_dev = argv.mode == "development";
    let is_serve = argv.env.as_ref().map_or(false, |e| e.WEBPACK_SERVE.is_some());

    let { allow_unused_variables = is_dev && is_serve, version } = options;

    Ok(Configuration {
        resolve: WebpackResolvePlugin::new(),
        module: ModuleRule::new(vec![ModuleRuleData::new("^.js$", "tsx")]),
        optimization: Optimization::new(Some(OptimizationOptions {
            remove_available_modules: true,
            minimizer: vec![
                EsbuildPlugin::new(Config {
                    target: String::from("es2022"),
                    minify: true,
                }),
            ],
        })),
        plugins: Plugins::new(vec![
            ProvidePlugin::new(plugins::ProvidePluginData::new([
                ("React", "react"),
                ("Buffer", [env!("BROWSER"), "buffer", "Buffer"]),
                ("process", env!("BROWSER")),
                ("setImmediate", env!("BROWSER")),
            ])),
            DefinePlugin::new(DefinePluginData {
                "ReactNull": None,
                "LICHTBLICK_SUITE_VERSION": version.into(),
                "API_URL": env!("API_URL").into(),
                "DEV_WORKSPACE": env!("DEV_WORKSPACE").into(),
                ...build_env_vars(),
            }),
            IgnorePlugin::new(IgnorePluginConfig::new(vec![IgnorePluginResourceData {
                resource_regex: regex::Regex::new(r".*locale$").unwrap(),
                context_regex: regex::Regex::new(r"moment$").unwrap(),
            }])),
            MonacoWebpackPlugin::new(MonacoWebpackPluginOptions {
                languages: vec!["typescript", "javascript"],
                filename: "[name].worker.[contenthash].js",
            }),
            ForkTsCheckerPlugin::new(ForkTsCheckerPluginConfig {
                typescript: TypescriptConfig {
                    config_file: Some(String::from(options.tsconfig_path.as_ref().unwrap_or_default())),
                    config_overwrite: Some(TypescriptOverwriteConfig {
                        compiler_options: TypeScriptCompilerOptions {
                            no_unused_locals: !allow_unused_variables,
                            no_unused_parameters: !allow_unused_variables,
                            jsx: is_dev && String::from("react-jsxdev"),
                        },
                    }),
                },
            }),
        ]),
        node: NodePlugin::new(NodePluginData {
            __dirname: true,
            __filename: true,
        }),
        ignore_warnings: vec![IgnoreWarning::new(IgnoreWarningData {
            module: regex::Regex::new(r"^node_modules\/typescript\/lib\/typescript\.js$").unwrap(),
            message: Regex::new(r"Critical dependency: the request of a dependency is an expression").unwrap(),
        })],
    })
}
```