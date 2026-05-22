```rust
use std::convert::{From, TryFrom};
use std::net::{SocketAddr, Url};

use actix_web::web;
use async_std::fs::File;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use crate::{AppConfiguration, AppSetting, IExtensionLoader, FoxgloveWebSocketDataSourceFactory};
use crate::dsfactory::IDataSourceFactory;
use crate::extloader::RemoteExtensionLoader;

pub struct WebRoot {
    extra_providers: Vec<web::Html>,
    data_sources: Option<Vec<IDataSourceFactory>>,
    app_configuration: AppConfiguration,
    extension_loaders: Vec<IExtensionLoader>,
}

impl WebRoot {
    pub async fn new() -> Self {
        let is_development = std::env::var("NODE_ENV").unwrap_or_default().eq("development");

        let default_extension_loaders: Vec<Box<dyn IExtensionLoader>> = vec![
            Box::new(StorageExtensionLoader {}),
            Box::new(LocalExtensionLoader {}),
        ];
        let workspace = std::env::args().collect::<Vec<String>>().into_iter()
            .find(|&arg| arg.starts_with("workspace="))
            .map(|arg| arg.split('=').nth(1).unwrap())
            .or(Some("default"));

        if let Some(workspace) = workspace {
            default_extension_loaders.push(Box::new(RemoteExtensionLoader::new(workspace)));
        }
        let extension_loaders: Vec<Box<dyn IExtensionLoader>> = default_extension_loaders.into_iter().collect();

        let data_sources = if let Some(data_sources) = self.data_sources.clone() {
            data_sources
        } else {
            vec![
                Ros1LocalBagDataSourceFactory {},
                Ros2LocalBagDataSourceFactory {},
                FoxgloveWebSocketDataSourceFactory {},
                RosbridgeDataSourceFactory {},
                UlogLocalDataSourceFactory {},
                SampleNuscenesDataSourceFactory {},
                McapLocalDataSourceFactory {},
                RemoteDataSourceFactory {},
            ]
        };

        Self {
            extra_providers: Vec::new(),
            data_sources,
            app_configuration: AppConfiguration {
                development: is_development,
                // Add any other defaults here
            },
            extension_loaders,
        }
    }

    pub async fn render(&self) -> Result<web::Html, Box<dyn std::error::Error>> {
        let url = &std::env::args().collect::<Vec<String>>()[1];
        let workspace = if let Some(workspace) = std::env::args().collect::<Vec<String>>()
            .into_iter()
            .find(|&arg| arg.starts_with("workspace="))
            .map(|arg| arg.split('=').nth(1).unwrap())
        {
            workspace
        } else {
            "default"
        };

        let shared_root = SharedRoot::new(
            self.enable_launch_preference_screen,
            vec![url.to_string()],
            self.data_sources.clone().unwrap_or_else(|| vec![
                Ros1LocalBagDataSourceFactory {},
                Ros2LocalBagDataSourceFactory {},
                FoxgloveWebSocketDataSourceFactory {},
                RosbridgeDataSourceFactory {},
                UlogLocalDataSourceFactory {},
                SampleNuscenesDataSourceFactory {},
                McapLocalDataSourceFactory {},
                RemoteDataSourceFactory {},
            ]),
            self.app_configuration.clone(),
            self.extension_loaders.clone(),
            self.enable_global_css,
            &self.extra_providers,
        );

        shared_root.render().await
    }
}

// Define other necessary types and functions here
```