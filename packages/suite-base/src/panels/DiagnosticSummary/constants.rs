```rust
use serde::{Deserialize, Serialize};

// Trim the message if it's too long. We sometimes get crazy massive messages here that can
// otherwise crash our entire UI. I looked at a bunch of messages manually and they are typically
// way smaller than 5KB, so this is a very generous maximum. But feel free to increase it more if
// necessary.
const MAX_STRING_LENGTH: usize = 5000; // 5KB

pub const DEFAULT_SECONDS_UNTIL_STALE: u8 = 5; // ROS rqt_runtime_monitor default

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Level {
    Ok,
    Warn,
    Error,
    Stale,
}

impl Default for Level {
    fn default() -> Self {
        Level::Ok
    }
}

pub const LEVEL_NAMES: &'static str = "ok warn error stale";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum KnownLevels {
    Ok(0),
    Warn(1),
    Error(2),
    Stale(3),
}

impl Default for KnownLevels {
    fn default() -> Self {
        KnownLevels::Ok
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AllowedDatatypes {
    DiagnosticArray(String),
}

impl Default for AllowedDatatypes {
    fn default() -> Self {
        AllowedDatatypes::DiagnosticArray(String::new())
    }
}

pub const MESSAGE_COLORS: &'static str = "success.main error.main warning.main text.secondary";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct DiagnosticSummaryConfig {
    pub min_level: Level,
    pub pinned_ids: Vec<String>,
    pub hardware_id_filter: String,
    pub topic_to_render: String,
    pub sort_by_level: bool,
}

impl Default for DiagnosticSummaryConfig {
    fn default() -> Self {
        DiagnosticSummaryConfig {
            min_level: Level::Ok,
            pinned_ids: Vec::new(),
            hardware_id_filter: "".to_string(),
            topic_to_render: "/diagnostics".to_string(),
            sort_by_level: true,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct SettingsTreeNodes {
    general: {
        label: String,
        fields: {
            topic_to_render: {
                label: String,
                input: "select",
                options: Vec<AllowedDatatypes>,
            } as SettingsTreeField & SettingsTreeFieldSelectString,
            sortByLevel: {
                label: String,
                input: "boolean" } as SettingsTreeField & SettingsTreeFieldBoolean,
            seconds_until_stale: {
                label: String,
                help: String,
                input: "number",
                placeholder: format!("{} seconds", DEFAULT_SECONDS_UNTIL_STALE),
                min: 0,
                step: 1,
                precision: 0,
            } as SettingsTreeField & SettingsTreeFieldNumber,
        },
    },
}
```