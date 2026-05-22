```rust
use anyhow::Result;
use bytes::{Buf, BytesMut};
use serde_json::{Value};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

pub struct Fixture {
    topics: Vec<(String, Value)>,
    capabilities: Vec<Value>,
}

impl From<&[(&str, &Value)]] for Fixture {
    fn from(topics: &[(&str, &Value)]) -> Self {
        Self {
            topics: topics
                .iter()
                .map(|(topic, schema)| (topic.to_string(), schema.clone()))
                .collect(),
            capabilities: Vec::new(),
        }
    }
}

pub struct PanelSetup {
    fixture: Fixture,
}

impl From<Fixture> for PanelSetup {
    fn from(fixture: Fixture) -> Self {
        Self { fixture }
    }
}

struct ThreeDeePanel {
    override_config: Value,
}

fn make_color(color_hex: &str, alpha: f64) -> Result<(u8, u8, u8, u8)> {
    let color_rgba = hex::decode(&color_hex[1..])?;
    if color_rgba.len() != 4 {
        return Err(anyhow!("Invalid color format"));
    }
    Ok((
        (color_rgba[0] as f64 / 255.0) * 255.0,
        (color_rgba[1] as f64 / 255.0) * 255.0,
        (color_rgba[2] as f64 / 255.0) * 255.0,
        alpha * 255.0,
    ))
}

fn pack_rviz_rgba(r: u8, g: u8, b: u8, a: u8) -> u32 {
    ((r << 24) | (g << 16) | (b << 8) | a) as _
}

struct MessageEvent<T> {
    topic: String,
    receive_time: Value,
    message: T,
    schema_name: String,
    size_in_bytes: u64,
}

fn write_point(view: &mut BytesMut, i: usize, x: f64, y: f64, z: f64, color_hex: &str) {
    let color_rgba = make_color(color_hex)?;
    view.put_u8(x as u8);
    view.put_u8(y as u8);
    view.put_u8(z as u8);
    view.put_u32(pack_rviz_rgba(color_rgba.0, color_rgba.1, color_rgba.2, color_rgba.3));
}

fn convert_to_point_cloud(point: &MessageEvent<Marker>) -> Result<PointCloud2> {
    let mut data = BytesMut::with_capacity(3 * 4);
    let mut view = bytes::BufMut::from(&mut data);

    write_point(&mut view, 0, point.message.points[0].x, point.message.points[0].y, point.message.points[0].z, "#f44336");
    write_point(&mut view, 1, point.message.points[1].x, point.message.points[1].y, point.message.points[1].z, "#4caf50");
    write_point(&mut view, 2, point.message.points[2].x, point.message.points[2].y, point.message.points[2].z, "#2196f3");

    let point_cloud = PointCloud2 {
        header: point.message.header.clone(),
        height: 1,
        width: 3,
        fields: vec![
            FieldData {
                name: "x".to_string(),
                offset: 0,
                datatype: Datatype::Float64,
                count: 1,
            },
            FieldData {
                name: "y".to_string(),
                offset: 8,
                datatype: Datatype::Float64,
                count: 1,
            },
            FieldData {
                name: "z".to_string(),
                offset: 16,
                datatype: Datatype::Float64,
                count: 1,
            },
            FieldData {
                name: "rgba".to_string(),
                offset: 24,
                datatype: Datatype::Int32,
                count: 1,
            },
        ],
        is_bigendian: false,
        point_step: 16,
        row_step: 3 * 16,
        data,
        is_dense: true,
    };

    Ok(point_cloud)
}

fn use_delayed_fixture(fixture: Fixture) -> Result<PanelSetup> {
    Ok(PanelSetup::from(fixture))
}

#[derive(Debug, Serialize)]
struct PointCloud2 {
    header: Header,
    height: u32,
    width: u32,
    fields: Vec<FieldData>,
    is_bigendian: bool,
    point_step: usize,
    row_step: usize,
    data: BytesMut,
    is_dense: bool,
}

#[derive(Debug, Serialize)]
struct FieldData {
    name: String,
    offset: usize,
    datatype: Datatype,
    count: u32,
}

#[derive(Debug, Serialize)]
enum Datatype {
    Float64,
    Int32,
}

#[derive(Debug, Serialize)]
struct Header {
    seq: u64,
    stamp: Timestamp,
    frame_id: String,
}

#[derive(Debug, Serialize)]
struct Timestamp {
    sec: u32,
    nsec: u32,
}

fn main() -> Result<()> {
    let topics = vec![
        ("markers", serde_json::to_value(json!({
            "header": {
                "seq": 0,
                "stamp": { "sec": 0, "nsec": 0 },
                "frame_id": "map"
            },
            "id": 0,
            "ns": "",
            "type": 1,
            "action": 0,
            "frame_locked": false,
            "pose": {
                "position": { "x": 1e7, "y": 0, "z": 0 },
                "orientation": { x: 0.010471, y: 0.008726, z: -0.000091, w: 0.999907 }
            },
            "scale": { "x": 0.017, "y": 0.017, "z": 0.017 },
            "color": make_color("#3f51b5", 0.25),
            "points": [
                { "x": 0, "y": 0.25, "z": 0 },
                { "x": 0.25, "y": -0.25, "z": 0 },
                { "x": -0.25, "y": -0.25, "z": 0 }
            ],
            "colors": [make_color("#f44336"), make_color("#4caf50"), make_color("#2196f3")],
            "lifetime": { "sec": 0, "nsec": 0 },
            "text": "",
            "mesh_resource": "",
            "mesh_use_embedded_materials": false
        })))),
        ("pointcloud", serde_json::to_value(json!({
            "header": {
                "seq": 0,
                "stamp": { "sec": 0, "nsec": 0 },
                "frame_id": "sensor"
            },
            "height": 1,
            "width": 3,
            "fields": [
                { "name": "x", offset: 0, datatype: 7, count: 1 },
                { "name": "y", offset: 4, datatype: 7, count: 1 },
                { "name": "z", offset: 8, datatype: 7, count: 1 },
                { "name": "rgba", offset: 12, datatype: 6, count: 1 }
            ],
            "is_bigendian": false,
            "point_step": 16,
            "row_step": 3 * 16,
            "data": bytes::BytesMut::with_capacity(3 * 4),
            "is_dense": true
        })))),
        ("tf", serde_json::to_value(json!({
            "topic": "/tf",
            "receiveTime": { "sec": 0, "nsec": 0 },
            "message": {
                "header": { "seq": 0, "stamp": { "sec": 0, "nsec": 0 }, "frame_id": "map" },
                "child_frame_id": "base_link",
                "transform": {
                    "translation": { x: 1e7, y: 0, z: 0 },
                    "rotation": QUAT_IDENTITY
                }
            },
            "schemaName": "geometry_msgs/TransformStamped",
            "sizeInBytes": 0
        }))),
    ];

    let fixture = Fixture {
        topics,
        capabilities: Vec::new(),
    };

    match use_delayed_fixture(fixture) {
        Ok(panel_setup) => println!("{:#?}", panel_setup),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
```