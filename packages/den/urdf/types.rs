```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Vector3 {
    pub x: f64;
    pub y: f64;
    pub z: f64;
}

#[derive(Serialize, Deserialize)]
pub struct Color {
    pub r: f64;
    pub g: f64;
    pub b: f64;
    pub a: f64;
}

#[derive(Serialize, Deserialize)]
pub struct Pose {
    pub xyz: Vector3;
    pub rpy: Vector3;
}

#[derive(Serialize, Deserialize)]
pub struct Inertia {
    ixx: f64;
    ixy: f64;
    ixz: f64;
    iyy: f64;
    iyz: f64;
    izz: f64,
}

#[derive(Serialize, Deserialize)]
pub enum JointType {
    Fixed,
    Continuous,
    Revolute,
    Planar,
    Prismatic,
    Floating,
}

#[derive(Serialize, Deserialize)]
pub struct UrdfInertial {
    origin: Pose;
    mass: f64;
    inertia: Inertia,
}

#[derive(Serialize, Deserialize)]
pub enum UrdfGeometryBox {
    GeometryType(String),
    Size(Vector3),
}

#[derive(Serialize, Deserialize)]
pub enum UrdfGeometryCylinder {
    GeometryType(String),
    Radius(f64),
    Length(f64),
}

#[derive(Serialize, Deserialize)]
pub enum UrdfGeometrySphere {
    GeometryType(String),
    Radius(f64),
}

#[derive(Serialize, Deserialize)]
pub enum UrdfGeometryMesh {
    GeometryType(String),
    Filename(String),
    Scale(Vector3),
}

#[derive(Serialize, Deserialize)]
pub type UrdfGeometry =
    | UrdfGeometryBox
    | UrdfGeometryCylinder
    | UrdfGeometrySphere
    | UrdfGeometryMesh;

#[derive(Serialize, Deserialize)]
pub struct UrdfCollider {
    name: Option<String>,
    origin: Pose,
    geometry: UrdfGeometry,
}

#[derive(Serialize, Deserialize)]
pub struct UrdfVisual = UrdfCollider & {
    material: Option<UrdfMaterial>,
};

#[derive(Serialize, Deserialize)]
pub struct UrdfLink {
    name: String,
    inertial: Option<UrdfInertial>,
    visuals: Vec<UrdfVisual>,
    colliders: Vec<UrdfCollider>,
}

#[derive(Serialize, Deserialize)]
pub struct UrdfJoint {
    name: String,
    joint_type: JointType,
    origin: Pose,
    parent: String,
    child: String,
    axis: Vector3,
    calibration: Option<Calibration>,
    dynamics: Option<Dynamics>,
    limit: Option<Limit>,
    mimic: Option<Mimic>,
    safety_controller: Option<SafetyController>,
}

#[derive(Serialize, Deserialize)]
pub struct Calibration {
    rising: f64,
    falling: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Dynamics {
    damping: f64,
    friction: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Limit {
    lower: f64,
    upper: f64,
    effort: f64,
    velocity: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Mimic {
    joint: String,
    multiplier: f64,
    offset: f64,
}

#[derive(Serialize, Deserialize)]
pub struct SafetyController {
    soft_lower_limit: f64,
    soft_upper_limit: f64,
    k_position: f64,
    k_velocity: f64,
}

#[derive(Serialize, Deserialize)]
pub struct UrdfMaterial {
    name: Option<String>,
    color: Option<Color>,
    texture: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UrdfRobot {
    name: String,
    links: std::collections::HashMap<String, UrdfLink>,
    joints: std::collections::HashMap<String, UrdfJoint>,
    materials: std::collections::HashMap<String, UrdfMaterial>,
}
```