```rust
use serde_json::{Map, Value};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use std::collections::HashMap;

// Define Rust structs and enums equivalent to TypeScript types

#[derive(Debug, Clone)]
pub struct Feature {
    pub geometry: GeoJsonObject,
    pub properties: Option<Map<String, Value>>,
}

#[derive(Debug, Clone)]
pub struct FeatureCollection {
    pub features: Vec<Feature>,
}

#[derive(Debug, Clone)]
pub enum GeoJsonObject {
    Point {
        coordinates: (f64, f64),
    },
    LineString {
        coordinates: Vec<(f64, f64)>,
    },
    Polygon {
        coordinates: Vec<Vec<(f64, f64)>>,
    },
    MultiPoint {
        coordinates: Vec<(f64, f64)>,
    },
    MultiLineString {
        coordinates: Vec<Vec<(f64, f64)>>,
    },
    MultiPolygon {
        coordinates: Vec<Vec<Vec<(f64, f64)>>>,
    },
}

pub struct PathOptions {
    // Define the fields of PathOptions as per your requirements
}

// Define Rust structs and enums equivalent to TypeScript types

#[derive(Debug, Clone)]
pub struct MessageEvent<F> {
    pub schema_name: F,
    pub message: Value,
}

#[derive(Debug, Clone)]
pub enum FoxgloveMessages {
    // Define the fields of FoxgloveMessages as per your requirements
}

// Implement Rust functions equivalent to TypeScript functions

fn has_fix(nav_sat_fix_msg: &Value) -> bool {
    let status = nav_sat_fix_msg.get("status");
    if let Some(status) = status.as_str() {
        match status {
            "GBAS_FIX" | "SBAS_FIX" | "FIX" => true,
            _ => false,
        }
    } else {
        false
    }
}

fn is_geo_json_message(msg_event: &MessageEvent<FoxgloveMessages>) -> bool {
    let datatype = msg_event.schema_name;
    matches!(
        datatype,
        FoxgloveMessages::GeoJSON | FoxgloveMessages::MsgGeoJson | "foxglove_msgs/GeoJSON" | "foxglove::GeoJSON",
    )
}

fn is_valid_map_message(msg_event: &MessageEvent<FoxgloveMessages>) -> bool {
    if is_geo_json_message(msg_event) {
        true
    } else {
        let msg = msg_event.message.clone();
        if let Some(nav_sat_fix_msg) = msg.as_object().and_then(|obj| obj.get("NavSatFix").cloned()) {
            let latitude = nav_sat_fix_msg.get("latitude");
            let longitude = nav_sat_fix_msg.get("longitude");
            latitude.is_f64() && longitude.is_f64()
        } else {
            false
        }
    }
}

fn is_supported_schema(schema_name: &str) -> bool {
    matches!(
        schema_name,
        "sensor_msgs/NavSatFix" | "sensor_msgs/msg/NavSatFix" | "ros.sensor_msgs.NavSatFix"
            | "foxglove_msgs/LocationFix"
            | "foxglove_msgs/msg/LocationFix"
            | "foxglove::LocationFix"
            | "foxglove_msgs/GeoJSON"
            | "foxglove_msgs/msg/GeoJSON"
            | "foxglove::GeoJSON",
    )
}

fn parse_geo_json(json: &str) -> Vec<(GeoJsonObject, HashMap<String, Value>)> {
    let parsed: serde_json::Value = serde_json::from_str(json).expect("Failed to parse JSON");
    match parsed {
        serde_json::Value::Array(arr) => arr.into_iter().map(|obj| (parse_geojson_object(obj), HashMap::new())).collect(),
        serde_json::Value::Object(obj) => obj.into_iter().map(|(key, value)| (parse_geojson_object(value), { key: key.to_string(), ..HashMap::new() })).collect(),
        _ => Vec::new(),
    }
}

fn parse_geojson_object(value: Value) -> GeoJsonObject {
    match value {
        serde_json::Value::Object(obj) => GeoJsonObject::Feature {
            geometry: Feature::Point {
                coordinates: obj.get("coordinates").unwrap().as_array().unwrap().clone().into_iter().map(|coord| (coord.as_f64().unwrap(), coord.as_f64().unwrap())).collect(),
            },
            properties: obj.get("properties").cloned(),
        },
        serde_json::Value::Array(arr) => GeoJsonObject::LineString {
            coordinates: arr.into_iter().map(|coord| (coord.as_f64().unwrap(), coord.as_f64().unwrap())).collect(),
        },
        serde_json::Value::Object(obj) => GeoJsonObject::Polygon {
            coordinates: obj.get("coordinates").unwrap().as_array().unwrap().into_iter().map(|polygon| polygon.into_iter().map(|inner| inner.into_iter().map(|coord| (coord.as_f64().unwrap(), coord.as_f64().unwrap())).collect()).collect()).collect(),
        },
        serde_json::Value::Array(arr) => GeoJsonObject::MultiPoint {
            coordinates: arr.into_iter().map(|coord| (coord.as_f64().unwrap(), coord.as_f64().unwrap())).collect(),
        },
        serde_json::Value::Array(arr) => GeoJsonObject::MultiLineString {
            coordinates: arr.into_iter().map(|polygon| polygon.into_iter().map(|inner| inner.into_iter().map(|coord| (coord.as_f64().unwrap(), coord.as_f64().unwrap())).collect()).collect()).collect(),
        },
        serde_json::Value::Array(arr) => GeoJsonObject::MultiPolygon {
            coordinates: arr.into_iter().map(|polygon| polygon.into_iter().map(|inner| inner.into_iter().map(|coord| (coord.as_f64().unwrap(), coord.as_f64().unwrap())).collect()).collect()).collect(),
        },
        _ => panic!("Unsupported JSON object type"),
    }
}
```