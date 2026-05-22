```rust
use std::net::Url;

use crate::{MemoryAppConfiguration, McapLocalBenchmarkDataSourceFactory, SyntheticDataSourceFactory};
use lichtblick_base::data_sources::IDataSourceFactory;
use lichtblick_base::players::{PointcloudPlayer, SinewavePlayer, TransformPlayer, TransformPreloadingPlayer};
use lichtblick_base::services::{StudioApp};

pub fn Root() -> impl 'static {
    let app_configuration = MemoryAppConfiguration::new({
        defaults: vec![(
            AppSetting::LAUNCH_PREFERENCE,
            LaunchPreferenceValue::WEB,
        ), (
            AppSetting::MESSAGE_RATE,
            240,
        )],
    });

    let data_sources = vec![
        McapLocalBenchmarkDataSourceFactory {},
        SyntheticDataSourceFactory::new("pointcloud", PointcloudPlayer, LAYOUTS::POINT_CLOUD_MULTIPLE_THREE_DEE),
        SyntheticDataSourceFactory::new("sinewave", SinewavePlayer, LAYOUTS::SINEWAVE),
        SyntheticDataSourceFactory::new("transform", TransformPlayer, LAYOUTS::MULTIPLE_THREE_DEE),
        SyntheticDataSourceFactory::new(
            "transformpreloading",
            TransformPreloadingPlayer,
            LAYOUTS::TRANSFORM_PRELOADING,
        ),
    ];

    let extension_loaders = vec![];

    SharedRoot {
        enable_launch_preference_screen: false,
        deep_links: Some(vec![url::Url::parse(&window.location.href).unwrap()]),
        data_sources: Box::new(data_sources),
        app_configuration: Box::new(app_configuration),
        extension_loaders: Box::new(extension_loaders),
        enable_global_css: true,
    }
}
```