```rust
use webpack::{Configuration, WebpackPluginInstance};

fn webpack_renderer_config(params: &WebpackConfigParams) -> Configuration {
    let is_dev = params.args.mode == "development";
    let is_serve = params.envs.WEBPACK_SERVE.unwrap_or(false);

    let allow_unused_variables = is_dev;

    let mut plugins: Vec<WebpackPluginInstance> = vec![];

    if is_serve {
        plugins.push(ReactRefreshPlugin {});
    }

    let app_config = make_config(params.args, params.env, allow_unused_variables, &params.package_json.version);

    let config: Configuration = {
        ..app_config
    };

    // force web target instead of electron-render
    // Fixes "require is not defined" errors if nodeIntegration is off
    // https://gist.github.com/msafi/d1b8571aa921feaaa0f893ab24bb727b
    config.target = "web";
    config.context = params.renderer_context.to_string();
    config.entry = params.renderer_entrypoint.to_string();

    let devtool = if is_dev {
        "eval-cheap-module-source-map"
    } else {
        params.prod_source_map.to_string()
    };

    config.output = OutputConfig {
        public_path: if is_serve {
            "/renderer/"
        } else {
            ""
        },
        path: PathBuf::from(params.output_path).join("renderer"),
    };

    config.optimization = Optimization {
        remove_available_modules: true,
        minimizer: vec![
            EsbuildPlugin {
                target: "es2022",
                minify: true,
            }
        ]
    };

    let mut plugins_vec = Vec::from(plugins);

    if let Some(app_plugins) = &app_config.plugins {
        plugins_vec.extend(app_plugins);
    }

    config.plugins = plugins_vec;

    config
}
```