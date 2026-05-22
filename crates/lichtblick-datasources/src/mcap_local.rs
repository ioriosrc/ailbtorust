// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use lichtblick_core::error::LichtblickError;
use lichtblick_mcap::source::McapIterableSource;
use lichtblick_players::iterable_player::{IterablePlayer, IterablePlayerConfig};
use lichtblick_players::Player;

use crate::factory::{DataSourceFactory, DataSourceFactoryArgs, DataSourceType, FormField};

/// Data source factory for local MCAP files.
pub struct McapLocalDataSourceFactory;

impl DataSourceFactory for McapLocalDataSourceFactory {
    fn id(&self) -> &str {
        "mcap-local-file"
    }

    fn display_name(&self) -> &str {
        "MCAP File"
    }

    fn description(&self) -> &str {
        "Open a local MCAP recording file"
    }

    fn source_type(&self) -> DataSourceType {
        DataSourceType::File
    }

    fn icon_name(&self) -> &str {
        "file"
    }

    fn supported_extensions(&self) -> Vec<&str> {
        vec!["mcap"]
    }

    fn initialize(&self, args: DataSourceFactoryArgs) -> Result<Box<dyn Player>, LichtblickError> {
        let data = args
            .file_data
            .ok_or_else(|| LichtblickError::DataSource("No file data provided".into()))?;

        let name = args.file_name.unwrap_or_else(|| "Unknown.mcap".into());

        let source = McapIterableSource::new(data);

        let config = IterablePlayerConfig {
            name,
            read_ahead_duration_ns: 120_000_000_000,
        };

        let mut player = IterablePlayer::new(source, config);
        player.initialize()?;

        Ok(Box::new(player))
    }
}
