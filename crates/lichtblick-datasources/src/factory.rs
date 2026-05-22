// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use lichtblick_core::error::LichtblickError;
use lichtblick_players::Player;
use serde::{Deserialize, Serialize};

/// Type of data source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DataSourceType {
    /// Opens a local file.
    File,
    /// Connects to a remote endpoint.
    Connection,
    /// Sample/demo data.
    Sample,
}

/// Arguments for initializing a data source.
pub struct DataSourceFactoryArgs {
    /// File data (for file-based sources).
    pub file_data: Option<Vec<u8>>,
    /// File name.
    pub file_name: Option<String>,
    /// Connection URL (for connection-based sources).
    pub url: Option<String>,
    /// Additional parameters.
    pub params: std::collections::HashMap<String, String>,
}

/// Form field configuration for data source dialogs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormField {
    pub id: String,
    pub label: String,
    pub field_type: FormFieldType,
    pub placeholder: Option<String>,
    pub default_value: Option<String>,
    pub required: bool,
}

/// Form field types.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FormFieldType {
    Text,
    Number,
    Toggle,
}

/// Trait for data source factories.
pub trait DataSourceFactory: Send + Sync {
    /// Unique identifier.
    fn id(&self) -> &str;

    /// Display name.
    fn display_name(&self) -> &str;

    /// Description.
    fn description(&self) -> &str;

    /// Source type.
    fn source_type(&self) -> DataSourceType;

    /// Icon name (for the UI).
    fn icon_name(&self) -> &str;

    /// Supported file extensions (for file sources).
    fn supported_extensions(&self) -> Vec<&str> {
        Vec::new()
    }

    /// Form configuration fields for connection parameters.
    fn form_config(&self) -> Vec<FormField> {
        Vec::new()
    }

    /// Create a player from the given arguments.
    fn initialize(&self, args: DataSourceFactoryArgs) -> Result<Box<dyn Player>, LichtblickError>;
}
