```rust
use std::time::{Duration, Instant};

pub struct Time {
    pub nanoseconds: u64,
}

pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub type Points = Vec<Point>;

pub struct MutablePoint2D {
    pub x: f32;
    pub y: f32,
}

pub type Point2D = ReadWrite<MutablePoint2D>;

pub struct Header {
    pub frame_id: String,
    pub stamp: Time,
    pub seq: i32,
}

pub type StampedMessage {
    pub header: Header,
}

pub type Duration = Time;

pub struct MutableOrientation {
    pub x: f32;
    pub y: f32;
    pub z: f32;
    pub w: f32,
}

pub type Scale = ReadWrite<MutableScale>;

pub type Color = ReadWrite<MutableColor>;

// NOTE: Deep mutability.
pub struct MutablePose {
    pub position: Point;
    pub orientation: MutableOrientation;
}

type Colors = Vec<Color>;

// Markers
#[derive(Clone, PartialEq)]
pub struct BaseMarker {
    pub ns: String,
    pub id: i32,
    action: u8,
    pose: MutablePose,
    scale: Scale,
    color: Option<Color>,
    colors: Option<Vec<Color>>,
    lifetime: Duration,
    frame_locked: bool,
    points: Option<Vec<Point>>,
    text: Option<String>,
    mesh_resource: String,
    mesh_use_embedded_materials: bool,
    primitive: String,
    metadata: Option<serde_json::Value>,
}

#[derive(Clone, PartialEq)]
pub struct MultiPointMarker {
    pub points: Points,
    colors: Option<Vec<Color>>,
}

#[derive(Clone, PartialEq)]
pub struct ArrowSize {
    pub shaft_length: Option<f32>;
    pub shaft_width: f32;
    pub head_length: f32;
    pub head_width: f32,
}

#[derive(Clone, PartialEq)]
pub struct ArrowMarker {
    type_: u8,
    points: Option<Vec<Point>>,
    size: Option<ArrowSize>,
}

#[derive(Clone, PartialEq)]
pub struct CubeMarker {
    type_: u8,
}

#[derive(Clone, PartialEq)]
pub struct SphereMarker {
    type_: u8,
}

#[derive(Clone, PartialEq)]
pub struct CylinderMarker {
    type_: u8,
}

#[derive(Clone, PartialEq)]
pub struct LineStripMarker {
    closed: bool,
    points: Points,
}

#[derive(Clone, PartialEq)]
pub struct LineListMarker {
    points: Points,
}

#[derive(Clone, PartialEq)]
pub struct CubeListMarker {
    points: Points,
}

#[derive(Clone, PartialEq)]
pub struct SphereListMarker {
    points: Points,
}

#[derive(Clone, PartialEq)]
pub struct PointsMarker {
    points: Points,
}

#[derive(Clone, PartialEq)]
pub struct TextMarker {
    type_: u8,
    text: String,
}

#[derive(Clone, PartialEq)]
pub struct MeshMarker {
    type_: u8,
    mesh_resource: String,
    mesh_use_embedded_materials: bool,
}

#[derive(Clone, PartialEq)]
pub struct TriangleListMarker {
    points: Points,
}

#[derive(Clone, PartialEq)]
pub struct InstancedLineListMarker {
    metadata_by_index: Option<Vec<serde_json::Value>>,
    scale_invariant: bool,
}

#[derive(Clone, PartialEq)]
pub struct ColorMarker {
    type_: u8,
}

pub type Marker = ArrowMarker | CubeMarker | CubeListMarker | SphereMarker | SphereListMarker | CylinderMarker | LineStripMarker | LineListMarker | PointsMarker | TextMarker | MeshMarker | TriangleListMarker | InstancedLineListMarker | ColorMarker;

#[derive(Clone, PartialEq)]
pub struct MarkerArray {
    markers: Vec<Marker>,
}

pub type PointField {
    name: String,
    offset: usize,
    datatype: u8,
    count: usize,
}

pub type PointCloud2 = StampedMessage {
    fields: Vec<PointField>,
    height: usize,
    width: usize,
    is_bigendian: bool,
    point_step: usize, // Length of point in bytes
    row_step: usize, // Length of row in bytes
    data: Vec<u8>,
    is_dense: Option<bool>,
    type_: u8,
}

pub type VelodynePacket = StampedMessage {
    stamp: Time,
    data: Vec<u8>, // 1206 bytes
};

pub type VelodyneScan = StampedMessage {
    packets: Vec<VelodynePacket>,
};
```