```rust
use std::path;

fn main() {
    // Define the configuration for the web server
    let dev_server_config = |params: ConfigParams| {
        WebpackDevServerConfiguration {
            static_options: StaticOptions {
                directory: path::PathBuf::from(params.outputPath),
            },
            history_api_fallback: params.historyApiFallback,
            hot: true,
            allowed_hosts: "all",
            headers: Some(Headers {
                "cross-origin-opener-policy": "same-origin",
                "cross-origin-embedder-policy": "credentialless",
            }),
            client: ClientOptions {
                overlay: Some(ClientOverlay {
                    runtime_errors: move |error| {
                        if let Err(e) = e.to_string().contains("Failed to execute 'importScripts' on 'WorkerGlobalScope'") {
                            return false;
                        }
                        true
                    },
                }),
            },
        }
    };

    // Define the main configuration
    let main_config = |params: ConfigParams, argv: WebpackArgv| -> Configuration {
        let is_dev = argv.mode == "development";
        let is_serve = argv.env?.WEBPACK_SERVE ?? false;

        let allow_unused_variables = is_dev;

        let mut plugins: Vec<webpack::Plugin> = Vec::new();

        if is_serve {
            plugins.push(ReactRefreshPlugin);
        }

        let app_webpack_config = make_config(env, argv, |allow_unused_variables| {
            version: params.version,
        });

        let config: Configuration = WebpackConfiguration {
            name: "main",

            // Apply the base configuration
            ..app_webpack_config,

            target: Target::Web,
            context: path::PathBuf::from(params.contextPath),
            entry: params.entrypoint,
            devtool: if is_dev { "eval-cheap-module-source-map" } else { params.prodSourceMap },

            output: WebpackOutputOptions {
                public_path: params.publicPath.unwrap_or("auto"),
                filename: if is_dev { "[name].js" } else { "[name].[contenthash].js" },
                path: path::PathBuf::from(params.outputPath),
            },

            plugins: vec![
                // Add the cleanWebpackPlugin
                CleanWebpackPlugin,
                // CopyPlugin to copy public files
                CopyPlugin {
                    patterns: vec![Pattern {
                        from: PathBuf::from("..").join("public"),
                    }],
                },
                // HtmlWebpackPlugin to generate index.html
                HtmlWebpackPlugin {
                    template_content: move |htmlWebpackPlugin| format!(
                        r#"<!doctype html>
  <html>
    <head>
      <meta charset="utf-8">
      <meta name="apple-mobile-web-app-capable" content="yes">
      {htmlWebpackPlugin.options.foxgloveExtraHeadTags}
      <style type="text/css" id="loading-styles">
        body {{
          margin: 0;
        }}
        #root {{
          height: 100vh;
          background-color: {};
          color: {};
        }}
        @media (prefers-color-scheme: dark) {{
          #root {{
            background-color: {};
            color: {};
          }}
        }}
      </style>
    </head>
    <body>
      <div id="root"></div>
    </body>
  </html>",
                        palette.light.background.default,
                        palette.light.text.primary,
                        palette.dark.background.default,
                        palette.dark.text.primary,
                    ),
                    foxglove_extra_head_tags: r#"
            <title>Lichtblick</title>
            <link rel="apple-touch-icon" sizes="180x180" href="apple-touch-icon.png" />
            <link rel="icon" type="image/png" sizes="32x32" href="favicon-32x32.png" />
            <link rel="icon" type="image/png" sizes="16x16" href="favicon-16x16.png" />
          "#,
                    // Add custom index.html options if provided
                    htmlWebpackPlugin_options: params.indexHtmlOptions,
                },
            ],
        },
    };

    // Return the configuration
    config
}
```