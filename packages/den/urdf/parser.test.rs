```rust
use std::collections::{HashMap, HashSet};

fn parse_urdf(xml: &str) -> Result<HashMap<String, Link>, Box<dyn std::error::Error>> {
    // Implementation goes here
    unimplemented!()
}

struct Link {
    name: String,
    visuals: Vec<Visual>,
    colliders: Vec<Collider>,
    materials: HashMap<String, Material>,
    joints: HashMap<String, Joint>,
}

struct Visual {
    geometry: Geometry,
    material: Option<Material>,
    name: String,
    origin: Origin,
}

struct Geometry {
    geometry_type: String,
    size: Size,
}

struct Size {
    x: f64,
    y: f64,
    z: f64,
}

struct Material {
    // Implementation goes here
    unimplemented!()
}

struct Origin {
    rpy: RPY,
    xyz: XYZ,
}

struct RPY {
    x: f64,
    y: f64,
    z: f64,
}

struct XYZ {
    x: f64,
    y: f64,
    z: f64,
}
```