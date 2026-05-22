// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use lichtblick_core::error::LichtblickError;
use lichtblick_players::websocket_player::FoxgloveWebSocketPlayer;
use lichtblick_players::Player;

use crate::factory::{DataSourceFactory, DataSourceFactoryArgs, DataSourceType, FormField, FormFieldType};

/// Data source factory for Foxglove WebSocket connections.
pub struct FoxgloveWebSocketDataSourceFactory;

impl DataSourceFactory for FoxgloveWebSocketDataSourceFactory {
    fn id(&self) -> &str {
        "foxglove-websocket"
    }

    fn display_name(&self) -> &str {
        "Foxglove WebSocket"
    }

    fn description(&self) -> &str {
        "Connect to a live ROS system via foxglove_bridge"
    }

    fn source_type(&self) -> DataSourceType {
        DataSourceType::Connection
    }

    fn icon_name(&self) -> &str {
        "websocket"
    }

    fn form_config(&self) -> Vec<FormField> {
        vec![FormField {
            id: "url".to_string(),
            label: "WebSocket URL".to_string(),
            field_type: FormFieldType::Text,
            placeholder: Some("ws://localhost:8765".to_string()),
            default_value: Some("ws://localhost:8765".to_string()),
            required: true,
        }]
    }

    fn initialize(&self, args: DataSourceFactoryArgs) -> Result<Box<dyn Player>, LichtblickError> {
        let url = args
            .url
            .or_else(|| args.params.get("url").cloned())
            .ok_or_else(|| LichtblickError::DataSource("No URL provided".into()))?;

        let player = FoxgloveWebSocketPlayer::new(url);
        Ok(Box::new(player))
    }
}
