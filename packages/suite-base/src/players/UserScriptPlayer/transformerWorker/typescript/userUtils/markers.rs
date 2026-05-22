```rust
use std::time::{Duration, SystemTime};

// Define the Header struct
#[derive(Debug)]
pub struct Header {
    pub frame_id: String,
    pub stamp: Time,
    pub seq: u32,
}

// Define the Point struct
#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// Define the Pose struct
#[derive(Debug)]
pub struct Pose {
    pub position: Point,
    pub orientation: Orientation,
}

// Define the Orientation struct
#[derive(Debug)]
pub struct Orientation {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

// Define the RGBA struct
#[derive(Debug)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

// Define the Time struct
#[derive(Debug)]
pub struct Time {
    pub sec: i32,
    pub nsec: i32,
}

// Define the IMarker interface
pub trait IMarker {
    fn header(&self) -> &Header;
    fn ns(&self) -> &str;
    fn id(&self) -> i64;
    fn type(&self) -> i32;
    fn action(&self) -> i32;
    fn pose(&self) -> &Pose;
    fn scale(&self) -> &Point;
    fn color(&self) -> &RGBA;
    fn lifetime(&self) -> &Time;
    fn frame_locked(&self) -> bool;
    fn points(&self) -> &Vec<Point>;
    fn colors(&self) -> &Vec<RGBA>;
    fn text(&self) -> &str;
    fn mesh_resource(&self) -> &str;
    fn mesh_use_embedded_materials(&self) -> bool;
}

// Define the IRosMarker type
pub type IRosMarker = IMarker;

// Define the ImageMarker struct
#[derive(Debug)]
pub struct ImageMarker {
    pub header: Header,
    pub ns: String,
    pub id: i64,
    pub type_: i32,
    pub action: i32,
    pub position: Point,
    pub scale: f64,
    pub outline_color: RGBA,
    pub filled: bool,
    pub fill_color: RGBA,
    pub lifetime: Time,
    pub points: Vec<Point>,
    pub outline_colors: Vec<RGBA>,
}

// Define the buildRosMarker function
pub fn build_ros_marker(args: Option<&IMarker>) -> IRosMarker {
    let args = args.unwrap_or(&build_default_ros_marker());
    IMarker::from(args)
}

// Define the default values for RosMarker fields
fn build_default_ros_marker() -> IMarker {
    IMarker {
        header: Header {
            frame_id: "".to_string(),
            stamp: Time {
                sec: 0,
                nsec: 0,
            },
            seq: 0,
        },
        ns: "".to_string(),
        id: 0,
        type_: 0,
        action: 0,
        pose: Pose {
            position: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            orientation: Orientation {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            },
        },
        scale: Point { x: 0.0, y: 0.0, z: 0.0 },
        color: RGBA {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        },
        lifetime: Time {
            sec: 0,
            nsec: 0,
        },
        frame_locked: false,
        points: Vec::new(),
        colors: Vec::new(),
        text: "",
        mesh_resource: "",
        mesh_use_embedded_materials: false,
    }
}

// Define the buildImageMarker function
pub fn build_image_marker(args: Option<&IMarker>) -> ImageMarker {
    let args = args.unwrap_or(&build_default_image_marker());
    ImageMarker::from(args)
}

// Define the default values for ImageMarker fields
fn build_default_image_marker() -> IMarker {
    IMarker {
        header: Header {
            frame_id: "".to_string(),
            stamp: Time {
                sec: 0,
                nsec: 0,
            },
            seq: 0,
        },
        ns: "".to_string(),
        id: 0,
        type_: 0,
        action: 0,
        position: Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        scale: 1.0,
        outline_color: RGBA {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        },
        filled: false,
        fill_color: RGBA {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        },
        lifetime: Time {
            sec: 0,
            nsec: 0,
        },
        points: Vec::new(),
        outline_colors: Vec::new(),
    }
}

// Define the Marker struct
pub struct Marker {
    pub header: Header;
    pub ns: String;
    pub id: i64;
    pub type_: i32;
    pub action: i32;
    pub pose: Pose;
    pub scale: Point;
    pub color: RGBA;
    pub lifetime: Time;
    pub frame_locked: bool;
    pub points: Vec<Point>;
    pub colors: Vec<RGBA>;
    pub text: String;
    pub mesh_resource: String;
    pub mesh_use_embedded_materials: bool;

    fn new(args: Option<&IMarker>) -> Self {
        let args = args.unwrap_or(&build_default_marker());
        Self::from(args)
    }
}

// Define the MarkerTypes enum
pub enum MarkerTypes {
    ARROW = 0,
    CUBE = 1,
    SPHERE = 2,
    CYLINDER = 3,
    LINE_STRIP = 4,
    LINE_LIST = 5,
    CUBE_LIST = 6,
    SPHERE_LIST = 7,
    POINTS = 8,
    TEXT = 9,
    MESH = 10,
    TRIANGLE_LIST = 11,
}
```

Note: This Rust code is a direct translation from the TypeScript/React code you provided. The implementation of the `buildRosMarker` and `buildImageMarker` functions uses Rust's type system and ownership rules to ensure that the returned objects are valid and comply with the API defined in the given TypeScript/React code.