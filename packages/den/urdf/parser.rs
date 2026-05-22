```rust
use std::collections::{HashMap, VecDeque};
use xml::Element;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

/// Represents a color in RGB format.
#[derive(Debug, Clone)]
struct Color {
    r: f64,
    g: f64,
    b: f64,
    a: f64,
}

/// Represents a pose with position and orientation.
#[derive(Debug, Clone)]
struct Pose {
    xyz: Vector3,
    rpy: Vector3,
}

/// Represents a vector in 3D space.
#[derive(Debug, Clone)]
struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

/// Represents a geometry shape.
enum UrdfGeometry {
    Box { size: Vector3 },
    Cylinder { length: f64, radius: f64 },
    Sphere { radius: f64 },
    Mesh { filename: String, scale: Vector3 },
}

/// Represents the type of joint.
#[derive(Debug, Clone)]
struct JointType {
    name: &'static str,
}

/// Represents an inertia tensor.
struct Inertia {
    ixx: f64,
    ixy: f64,
    ixz: f64,
    iyy: f64,
    iyz: f64,
    izz: f64,
}

/// Represents a material with color and texture.
#[derive(Debug, Clone)]
struct UrdfMaterial {
    name: &'static str,
    color: Option<Color>,
    texture: Option<String>,
}

/// Represents a link in the robot model.
#[derive(Debug, Clone)]
struct UrdfLink {
    name: &'static str,
    visuals: Vec<UrdfVisual>,
    colliders: Vec<UrdfCollider>,
    inertial: UrdfInertial,
}

/// Represents an inertial property of a link.
#[derive(Debug, Clone)]
struct UrdfInertial {
    origin: Pose,
    mass: f64,
    inertia: Inertia,
}

/// Represents a joint in the robot model.
#[derive(Debug, Clone)]
struct UrdfJoint {
    name: &'static str,
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

/// Represents a visual element of a link.
#[derive(Debug, Clone)]
struct UrdfVisual {
    name: &'static str,
    origin: Pose,
    geometry: UrdfGeometry,
    material: Option<UrdfMaterial>,
}

/// Represents a collision element of a link.
#[derive(Debug, Clone)]
struct UrdfCollider {
    name: &'static str,
    origin: Pose,
    geometry: UrdfGeometry,
}

/// Represents the dynamics properties of a joint.
#[derive(Debug, Clone)]
struct Dynamics {
    damping: f64,
    friction: f64,
}

/// Represents the limit properties of a joint.
#[derive(Debug, Clone)]
struct Limit {
    lower: f64,
    upper: f64,
    effort: f64,
    velocity: f64,
}

/// Represents the mimic properties of a joint.
#[derive(Debug, Clone)]
struct Mimic {
    joint: &'static str,
    multiplier: f64,
    offset: f64,
}

/// Represents the safety controller properties of a joint.
#[derive(Debug, Clone)]
struct SafetyController {
    soft_lower_limit: f64,
    soft_upper_limit: f64,
    k_position: f64,
    k_velocity: f64,
}

fn parseUrdf(xml: Element) -> UrdfRobot {
    let name = xml.attr("name").unwrap_or_default();
    if name.is_empty() {
        panic!("missing attribute 'name' in {}", xml);
    }
    let mut links = HashMap::new();
    let mut joints = HashMap::new();
    let mut materials = HashMap::new();

    for child in xml.children() {
        match &child.name().unwrap_or_default() {
            "robot" => links.extend(parseRobot(child)),
            "link" => links.insert(parseLink(child).name, parseLink(child)),
            "joint" => joints.insert(parseJoint(child).name, parseJoint(child)),
            "material" => materials.insert(parseMaterial(child).name, parseMaterial(child)),
            _ => continue,
        }
    }

    UrdfRobot {
        name,
        links,
        joints,
        materials,
    }
}

fn parseRobot(xml: Element) -> HashMap<&str, UrdfLink> {
    let mut result = HashMap::new();
    for child in xml.children() {
        if let Some(link) = parseLink(child) {
            result.insert(link.name, link);
        }
    }
    result
}

fn parseLink(xml: Element) -> UrdfLink {
    let name = xml.attr("name").unwrap_or_default();
    if name.is_empty() {
        panic!("missing attribute 'name' in {}", xml);
    }
    let mut visuals = Vec::new();
    let mut colliders = Vec::new();
    let inertial = parseInertial(xml);
    for child in xml.children() {
        match &child.name().unwrap_or_default() {
            "visual" => visuals.push(parseVisual(child)),
            "collision" => colliders.push(parseCollision(child)),
            _ => continue,
        }
    }
    UrdfLink {
        name,
        visuals,
        colliders,
        inertial,
    }
}

fn parseVisual(xml: Element) -> UrdfVisual {
    let name = xml.attr("name").unwrap_or_default();
    if name.is_empty() {
        panic!("missing attribute 'name' in {}", xml);
    }
    let origin = parsePose(xml);
    let geometry = parseGeometry(xml);
    let material = match xml.children().find(|&child| child.name() == Some("material")) {
        Some(material) => Some(parseMaterial(material)),
        None => None,
    };
    UrdfVisual { name, origin, geometry, material }
}

fn parseCollision(xml: Element) -> UrdfCollider {
    let name = xml.attr("name").unwrap_or_default();
    if name.is_empty() {
        panic!("missing attribute 'name' in {}", xml);
    }
    let origin = parsePose(xml);
    let geometry = parseGeometry(xml);
    UrdfCollider { name, origin, geometry }
}

fn parseInertial(xml: Element) -> UrdfInertial {
    let mut result = Pose::default();
    for child in xml.children() {
        if let Some(inertial) = parseInertialChild(child) {
            result += inertial;
        }
    }
    Inertial {
        origin: result,
        mass: parseMass(xml),
        inertia: parseInertia(xml),
    }
}

fn parseInertialChild(xml: Element) -> Option<UrdfInertial> {
    let name = xml.name().unwrap_or_default();
    match &name {
        "origin" => Some(parsePose(xml)),
        "mass" => Some(parseMass(xml)),
        "inertia" => Some(parseInertia(xml)),
        _ => None,
    }
}

fn parseMass(xml: Element) -> f64 {
    let value = xml.text().unwrap_or_default();
    if !value.parse::<f64>().is_ok() {
        panic!("expected float value in \"{}\"", xml);
    }
    value.parse::<f64>()?.abs()
}

fn parseGeometry(xml: Element) -> UrdfGeometry {
    if xml.children().len() < 1 {
        panic!("<geometry> must contain box, cylinder, sphere, or mesh");
    }

    let child = xml.children().next().unwrap();
    match &child.name().unwrap_or_default() {
        "box" => parseBoxGeometry(child),
        "cylinder" => parseCylinderGeometry(child),
        "sphere" => parseSphereGeometry(child),
        "mesh" => parseMeshGeometry(child),
        _ => panic!("<geometry> must contain box, cylinder, sphere, or mesh"),
    }
}

fn parseBoxGeometry(xml: Element) -> UrdfGeometry {
    let size = parseVec3Attribute(xml, "size");
    if size.is_none() {
        panic!("missing attribute 'size' in {}", xml);
    }
    UrdfGeometry::Box { size }
}

fn parseCylinderGeometry(xml: Element) -> UrdfGeometry {
    let length = parseFloatAttribute(xml, "length");
    let radius = parseFloatAttribute(xml, "radius");
    UrdfGeometry::Cylinder { length, radius }
}

fn parseSphereGeometry(xml: Element) -> UrdfGeometry {
    let radius = parseFloatAttribute(xml, "radius");
    UrdfGeometry::Sphere { radius }
}

fn parseMeshGeometry(xml: Element) -> UrdfGeometry {
    let filename = xml.attr("filename").unwrap_or_default();
    let scale = parseVec3Attribute(xml, "scale");
    if filename.is_empty() {
        panic!("missing attribute 'filename' in {}", xml);
    }
    UrdfGeometry::Mesh { filename, scale }
}

fn parsePose(xml: Element) -> Pose {
    let xyz = parseVec3Attribute(xml, "xyz").unwrap_or_default();
    let rpy = parseVec3Attribute(xml, "rpy").unwrap_or_default();
    Pose { xyz, rpy }
}

fn parseVec3Attribute(xml: Element, attrib_name: &str) -> Option<Vector3> {
    let parts = xml.attr(attrib_name).unwrap_or_default().trim().split(/\s+/);
    if parts.len() != 3 {
        return None;
    }

    let [x, y, z] = parts.map(|s| s.parse::<f64>().unwrap()).collect::<Vec<_>>();
    Some(Vector3 { x, y, z })
}

fn parseColorAttribute(xml: Element, attrib_name: &str) -> Option<Color> {
    let parts = xml.attr(attrib_name).unwrap_or_default().trim().split(/\s+/);
    if parts.len() != 4 {
        return None;
    }

    let [r, g, b, a] = parts.map(|s| s.parse::<f64>().unwrap()).collect::<Vec<_>>();
    Some(Color { r, g, b, a })
}

fn parseMass(xml: Element) -> f64 {
    xml.text().unwrap_or_default().parse::<f64>().unwrap()
}

fn parseInertia(xml: Element) -> Inertia {
    let ixx = parseFloatAttribute(xml, "ixx");
    let ixy = parseFloatAttribute(xml, "ixy");
    let ixz = parseFloatAttribute(xml, "ixz");
    let iyy = parseFloatAttribute(xml, "iyy");
    let iyz = parseFloatAttribute(xml, "iyz");
    let izz = parseFloatAttribute(xml, "izz");
    Inertia { ixx, ixy, ixz, iyy, iyz, izz }
}

fn main() {
    // Example usage
    let xml = "<robot name='example'>...</robot>";
    let robot = parseUrdf(Element::from_str(xml).unwrap());
    println!("{:?}", robot);
}
```