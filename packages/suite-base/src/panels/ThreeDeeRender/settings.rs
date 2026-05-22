```rust
use serde::{Deserialize, Serialize};
use std::ops::Mul;

#[derive(Serialize, Deserialize, PartialEq)]
pub struct SelectEntry {
    pub label: String;
    pub value: String;
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct TwoColors(pub Vec3);
impl From<[f64; 2]> for TwoColors {
    fn from([r, g]: [f64; 2]) -> Self {
        Self(r.mul(255.0).round() as u8 / 255.0, g.mul(255.0).round() as u8 / 255.0)
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Vec3(pub [f64; 3]);

// Common settings for all persisted SceneExtension settings.
#[derive(Serialize, Deserialize, PartialEq)]
pub struct BaseSettings {
    #[serde(default = "true")]
    pub visible: bool;
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_locked: Option<bool>;
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct LayerSettingsEntity {
    #[serde(flatten)]
    pub base_settings: BaseSettings,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_outlines: Option<bool>;
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>;
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_id_variable: Option<String>;
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct CustomLayerSettings {
    #[serde(flatten)]
    pub base_settings: BaseSettings,
    pub instance_id: String,
    pub layer_id: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
}

const PRECISION_DISTANCE: f64 = 0.001;
const PRECISION_DEGREES: f64 = 0.5;

pub fn field_size(label: &str, value: Option<f64>, placeholder: Option<f64>) -> SettingsTreeField {
    SettingsTreeField {
        label,
        input: "number",
        min: 0.0,
        step: 0.5,
        precision: PRECISION_DISTANCE,
        value: value.map(|v| v.to_string()),
        placeholder: placeholder.map(|p| p.to_string()),
    }
}

pub fn field_scale_vec3(label: &str, value: Vec3) -> SettingsTreeField {
    SettingsTreeField {
        label,
        input: "vec3",
        labels: ["X", "Y", "Z"],
        step: 0.5,
        precision: PRECISION_DISTANCE,
        value: value.into_iter().map(|v| v.to_string()).collect(),
    }
}

pub fn field lineWidth(label: &str, value: Option<f64>, placeholder: Option<f64>) -> SettingsTreeField {
    SettingsTreeField {
        label,
        input: "number",
        min: 0.0,
        step: 0.005,
        precision: 4,
        value: value.map(|v| v.to_string()),
        placeholder: placeholder.map(|p| p.to_string()),
    }
}

pub fn field_gradient(label: &str, value: TwoColors) -> SettingsTreeField {
    SettingsTreeField { label, input: "gradient", value: serde_json::to_string(&value).unwrap() }
}
```