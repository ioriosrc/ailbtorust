// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

//! Extension types and data structures.

use serde::{Deserialize, Serialize};

/// Extension format type.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExtensionFormat {
    /// Legacy Lichtblick/Foxglove .foxe format (ZIP with JS bundle)
    Legacy,
    /// Future native Rust/WASM extension format
    Native,
}

/// Metadata describing an installed extension.
/// Compatible with Lichtblick Node.js ExtensionInfo.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExtensionInfo {
    /// Unique ID: "{publisher}.{name}"
    pub id: String,
    /// Extension name (from package.json)
    pub name: String,
    /// Publisher name
    pub publisher: String,
    /// Display name for UI
    pub display_name: String,
    /// Version string
    pub version: String,
    /// Description
    pub description: String,
    /// License
    pub license: String,
    /// Homepage URL
    pub homepage: String,
    /// Keywords/tags
    pub keywords: Vec<String>,
    /// README content (markdown)
    pub readme: String,
    /// CHANGELOG content (markdown)
    pub changelog: String,
    /// Size of the extension archive in bytes
    pub size: usize,
    /// Extension format
    pub format: ExtensionFormat,
}

/// Contribution points registered by an extension when activated.
#[derive(Clone, Debug, Default)]
pub struct ContributionPoints {
    /// Panels registered by this extension.
    /// Key: fully qualified panel ID ("{extensionName}.{panelName}")
    pub panels: Vec<RegisteredPanel>,
    /// Message converters registered by this extension.
    pub message_converters: Vec<MessageConverter>,
    /// Topic alias functions registered by this extension.
    pub topic_aliases: Vec<TopicAlias>,
}

/// A panel registered by an extension.
#[derive(Clone, Debug)]
pub struct RegisteredPanel {
    /// Fully qualified panel ID: "{extensionDisplayName}.{panelName}"
    pub id: String,
    /// Panel name (as registered by the extension)
    pub name: String,
    /// Extension ID that registered this panel
    pub extension_id: String,
    /// Extension display name
    pub extension_name: String,
}

/// A message converter registered by an extension.
#[derive(Clone, Debug)]
pub struct MessageConverter {
    /// Source schema name
    pub from_schema: String,
    /// Target schema name
    pub to_schema: String,
    /// Extension ID that registered this converter
    pub extension_id: String,
    /// Converter identifier
    pub converter_id: String,
}

/// A topic alias function registered by an extension.
#[derive(Clone, Debug)]
pub struct TopicAlias {
    /// Extension ID that registered this alias
    pub extension_id: String,
}

/// Files extracted from a .foxe archive.
#[derive(Clone, Debug)]
pub struct FoxeContents {
    /// package.json content
    pub package_json: String,
    /// dist/extension.js content (the JS bundle)
    pub extension_js: String,
    /// README.md content (optional)
    pub readme: String,
    /// CHANGELOG.md content (optional)
    pub changelog: String,
}

/// Parsed package.json from a .foxe extension.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PackageJson {
    pub name: String,
    #[serde(default)]
    pub version: String,
    #[serde(default, rename = "displayName")]
    pub display_name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub publisher: String,
    #[serde(default)]
    pub license: String,
    #[serde(default)]
    pub homepage: String,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub main: String,
}

/// Stored extension data (metadata + pre-extracted JS source).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoredExtension {
    pub info: ExtensionInfo,
    /// Pre-extracted extension JS source code (dist/extension.js).
    /// Stored directly — no re-decompression needed.
    pub extension_js: String,
}
