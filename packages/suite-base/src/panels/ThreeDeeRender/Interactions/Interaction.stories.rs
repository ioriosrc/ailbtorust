```rust
use serde_json::json;

fn main() {
    // Define a simple Rust struct for the PointCloud2 message
    #[derive(Serialize, Deserialize)]
    pub struct PointCloud2 {
        fields: Vec<Fields>,
        type_: i32,
        pose: Pose,
        header: Header,
        height: i32,
        is_bigendian: bool,
        is_dense: bool,
        point_step: i32,
        row_step: i32,
        width: i32,
        data: Vec<u8>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Fields {
        name: String,
        offset: i32,
        datatype: i32,
        count: i32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Pose {
        position: Position,
        orientation: Orientation,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Position {
        x: f64,
        y: f64,
        z: f64,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Orientation {
        x: f64,
        y: f64,
        z: f64,
        w: f64,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Header {
        seq: i32,
        frame_id: String,
        stamp: Stamp,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Stamp {
        sec: i32,
        nsec: i32,
    }

    fn main() {
        let marker_object = json!({
            "id": "12345",
            "header": { "frame_id": "some_frame", "stamp": { "sec": 0, "nsec": 0 } },
            "action": 0,
            "ns": "",
            "text": "hello\nthere",
            "type": 0,
            "scale": { "x": 2.0, "y": 2.0, "z": 4.0 },
            "orientation": {
                "x": 0.0,
                "y": 0.0,
                "z": Math::sin(std::f64::consts::PI / 8.0),
                "w": std::f64::cos(std::f64::consts::PI / 8.0)
            },
            "color": { "r": 1.0, "g": 0.1, "b": 0, "a": 0.7 },
            "pose": {
                "position": { "x": -1.0, "y": 1.0, "z": -5.0 },
                "orientation": { "x": 0.0, "y": 0.0, "z": 0.0, "w": 1.0 }
            }
        });

        let point_cloud_message = json!({
            "fields": [
                {"name": "x", "offset": 0, "datatype": 7, "count": 1},
                {"name": "y", "offset": 4, "datatype": 7, "count": 1},
                {"name": "z", "offset": 8, "datatype": 7, "count": 1},
                {"name": "rgb", "offset": 16, "datatype": 7, "count": 1},
            ],
            "type": 102,
            "pose": {
                "position": { "x": 0.0, "y": 0.0, "z": 0.0 },
                "orientation": { "x": 0.0, "y": 0.0, "z": 0.0, "w": 0.0 }
            },
            "header": {
                "seq": 0,
                "frame_id": "root_frame_id",
                "stamp": { "sec": 10, "nsec": 10 }
            },
            "height": 1,
            "is_bigendian": false,
            "is_dense": 1,
            "point_step": 32,
            "row_step": 32,
            "width": 2,
            "data": new Uint8Array([
                // point 1
                125, 236, 11, 197,
                118, 102, 48, 196,
                50, 194, 23, 192,
                0, 0, 128, 63,
                10, 255, 230, 127,
                254, 127, 0, 0, 16, 142, 140, 0, 161, 254, 127, 0,
                // point 2
                125, 236, 11, 197,
                118, 102, 48, 196,
                50, 194, 23, 192,
                9, 8, 7, 6,
            ])
        });

        let point_cloud_with_additional_fields = json!({
            "fields": [
                {"name": "x", "offset": 0, "datatype": 7, "count": 1},
                {"name": "y", "offset": 4, "datatype": 7, "count": 1},
                {"name": "z", "offset": 8, "datatype": 7, "count": 1},
                {"name": "rgb", "offset": 16, "datatype": 7, "count": 1},
            ],
            "type": 102,
            "pose": {
                "position": { "x": 0.0, "y": 0.0, "z": 0.0 },
                "orientation": { "x": 0.0, "y": 0.0, "z": 0.0, "w": 0.0 }
            },
            "header": {
                "seq": 0,
                "frame_id": "root_frame_id",
                "stamp": { "sec": 10, "nsec": 10 }
            },
            "height": 1,
            "is_bigendian": false,
            "is_dense": 1,
            "point_step": 32,
            "row_step": 32,
            "width": 2,
            "data": new Uint8Array([
                // point 1
                125, 236, 11, 197,
                118, 102, 48, 196,
                50, 194, 23, 192,
                0, 0, 128, 63,
                10, 255, 230, 127,
                254, 127, 0, 0, 16, 142, 140, 0, 161, 254, 127, 0,
                // point 2
                125, 236, 11, 197,
                118, 102, 48, 196,
                50, 194, 23, 192,
                9, 8, 7, 6,
            ])
        });

        // Create an instance of Interactions with the shared props
        let mut interactions = Interactions {
            selected_object: None,
            interactions_tab_type: OBJECT_TAB_TYPE,
            set_interactions_tab_type: |_tab| {},
        };

        // Render different components based on the story name
        if std::env::args().nth(1).unwrap() == "Default" {
            // Render without clicked object
            let component = Default::render(&interactions);
            println!("Rendered Default component:");
            println!("{:?}", component);
        } else if std::env::args().nth(1).unwrap() == "PointCloud" {
            // Render with interactionData and point color
            let cloud1 = {
                selected_object.object.clone(),
                POINT_CLOUD_MESSAGE.clone()
            };
            let cloud2 = {
                selected_object.object.clone(),
                POINT_CLOUD_WITH_ADDITIONAL_FIELDS.clone()
            };

            let component = PointCloud::render(&interactions, &cloud1, &cloud2);
            println!("Rendered PointCloud component:");
            println!("{:?}", component);
        } else {
            eprintln!("Unsupported story name");
        }
    }
}
```