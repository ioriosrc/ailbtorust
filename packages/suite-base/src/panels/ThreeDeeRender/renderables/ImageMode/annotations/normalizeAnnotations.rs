```rust
use std::collections::HashMap;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use rostime::{Time, from_nanosec};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Serialize, Deserialize, PartialEq)]
struct Circle {
    center_x: f64,
    center_y: f64,
    radius: f64,
    fill_color: Option<[u8; 4]>,
    outline_color: Option<[u8; 4]>,
    scale: f32,
}

#[derive(Serialize, Deserialize, PartialEq)]
struct Text {
    position: [f64; 2],
    text: String,
    text_color: Option<[u8; 4]>,
    background_color: Option<[u8; 4]>,
    font_size: f32,
    padding: f32,
}

#[derive(Serialize, Deserialize, PartialEq)]
struct ImageMarker {
    header: rostime::Header,
    type_: String,
    scale: f32,
    points: Vec<Point>,
    outline_colors: Option<Vec<Option<[u8; 4]>>>,
    filled: bool,
    fill_color: Option<[u8; 4]>,
}

#[derive(Serialize, Deserialize, PartialEq)]
struct ImageMarkerArray {
    markers: Vec<ImageMarker>,
}

#[derive(Serialize, Deserialize, PartialEq)]
struct Annotation {
    type_: String,
    stamp: rostime::Time,
    position: [f64; 2],
    fill_color: Option<[u8; 4]>,
    outline_color: Option<[u8; 4]>,
    radius: f32,
    thickness: f32,
    points: Vec<Point>,
    outline_colors: Option<Vec<Option<[u8; 4]>>>,
    filled: bool,
    fill_color: Option<[u8; 4]>,
}

fn normalize_timestamp(stamp: rostime::Time | i64) -> rostime::Time {
    if stamp.is_i64() {
        from_nanosec(stamp as i64)
    } else {
        stamp
    }
}

fn filter_map<T, U>(
    list: Vec<T>,
    mut callback: impl FnMut(T, usize) -> Option<U>,
) -> Vec<U> {
    let mut results = Vec::new();
    for (index, item) in list.iter().enumerate() {
        if let Some(result) = callback(item, index) {
            results.push(result);
        }
    }
    results
}

fn image_marker_type_to_style(type_: String) -> Option<&'static str> {
    match type_.as_str() {
        "LINE_LIST" | "LINE_STRIP" | "POINTS" | "POLYGON" => Some("points"),
        _ => None,
    }
}

fn normalize_ros_image_marker(message: ImageMarker, message_path: &[String]) -> Annotation {
    let mut annotation = Annotation {
        type_: String::from(&message.type_),
        stamp: message.header.stamp,
        position: [message.center_x as f64, message.center_y as f64],
        fill_color: message.filled.then(|| {
            if message.fill_color.is_empty() {
                None
            } else {
                Some([message.fill_color[0], message.fill_color[1], message.fill_color[2], 255])
            }
        }),
        outline_color: message.outline_color.map(|color| {
            if color.is_empty() {
                None
            } else {
                Some([
                    color[0],
                    color[1],
                    color[2],
                    color.len() as u8,
                ])
            }
        }),
        radius: message.scale,
        thickness: 1.0, // Assuming default thickness for points
        points: message.points,
        outline_colors: message.outline_colors.map(|colors| {
            colors.into_iter().flatten().collect()
        }),
        filled: message.filled,
        fill_color: message.fill_color.map(|color| {
            if color.is_empty() {
                None
            } else {
                Some([
                    color[0],
                    color[1],
                    color[2],
                    255,
                ])
            }
        }),
    };

    annotation
}

fn normalize_annotations(message: &serde_json::Value, datatype: &str) -> Vec<Annotation> {
    match datatype {
        "visualization_msgs/ImageMarker" => vec![normalize_ros_image_marker(
            message.as_object().unwrap()["header"].as_object().unwrap(),
            &[String::from("header"), String::from("stamp")],
        )],
        "visualization_msgs/msg/ImageMarker" => vec![normalize_ros_image_marker(
            message.as_object().unwrap()["header"].as_object().unwrap(),
            &[String::from("header"), String::from("stamp")],
        )],
        "ros.visualization_msgs.ImageMarker" => vec![normalize_ros_image_marker(
            message.as_object().unwrap()["header"].as_object().unwrap(),
            &[String::from("header"), String::from("stamp")],
        )],
        "foxglove_msgs/ImageMarkerArray" => normalize_ros_image_marker_array(message),
        "foxglove_msgs/msg/ImageMarkerArray" => normalize_ros_image_marker_array(message),
        "studio_msgs/ImageMarkerArray" | "studio_msgs/msg/ImageMarkerArray" | "visualization_msgs/ImageMarkerArray"
        | "visualization_msgs/msg/ImageMarkerArray"
        | "ros.visualization_msgs.ImageMarkerArray" => {
            let markers = message.as_object().unwrap()["markers"].as_array().unwrap();
            markers.iter().map(|marker| normalize_ros_image_marker(marker.as_object().unwrap(), &["markers", 0])).collect()
        }
        "webviz_msgs/ImageMarkerArray" => vec![],
        "foxglove_msgs/ImageAnnotations" | "foxglove_msgs/msg/ImageAnnotations" | "foxglove::ImageAnnotations"
        | "foxglove.ImageAnnotations" => {
            let annotations = message.as_array().unwrap();
            annotations.iter().map(|annotation| match annotation.as_object().unwrap()["type"].as_str().unwrap() {
                "LINE_LIST" | "LINE_STRIP" | "POINTS" | "POLYGON" => normalize_ros_image_marker(
                    annotation.as_object().unwrap(),
                    &[String::from("header"), String::from("stamp")],
                ),
                _ => Annotation {
                    type_: String::from("text"),
                    stamp: rostime::Time::now(),
                    position: [0.0, 0.0],
                    text: "Unknown annotation type".to_string(),
                    text_color: None,
                    background_color: None,
                    font_size: DEFAULT_FONT_SIZE as f32,
                    padding: DEFAULT_PADDING as f32,
                },
            }).collect()
        }
        _ => vec![],
    }
}

fn get_annotationAtPath(message: &serde_json::Value, path: &[String]) -> serde_json::Value {
    let mut value = message;
    for key in path {
        if let Some(obj) = value.as_object_mut() {
            value = obj.get(key).unwrap();
        } else {
            return serde_json::from_value(serde_json::Map::new()).unwrap();
        }
    }
    value.clone()
}

pub fn normalize_annotations(message: &serde_json::Value, datatype: &str) -> Vec<Annotation> {
    normalize_annotations(message, datatype)
}
```