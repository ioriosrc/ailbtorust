```rust
use std::collections::{HashSet, VecDeque};

// Define a function to add Foxglove schema variations to a set
fn add_foxglove_schema(output: &mut HashSet<String>, data_type: &str) {
    // Add the Foxglove json, protobuf, and flatbuffer variation: foxglove.PointCloud
    output.insert(data_type.to_string());

    // Split the data type into parts and extract the leaf part
    let parts: Vec<&str> = data_type.split('.').collect();
    if parts.len() < 2 {
        panic!("Invalid Foxglove schema: {}", data_type);
    }
    let leaf = parts[1..].join("/");

    // Add the ROS1 variation: foxglove_msgs/PointCloud
    output.insert(format!("foxglove_msgs/{leaf}"));

    // Add the ROS2 variation: foxglove_msgs/msg/PointCloud
    output.insert(format!("foxglove_msgs/msg/{leaf}"));

    // Add the IDL variation: foxglove::PointCloud
    output.insert(format!("foxglove::{leaf}"));
}

fn main() {
    let mut frame_transform_datatypes = HashSet::new();
    add_foxglove_schema(&mut frame_transform_datatypes, "foxglove.FrameTransform");

    let mut frame_transforms_datatypes = HashSet::new();
    add_foxglove_schema(&mut frame_transforms_datatypes, "foxglove.FrameTransforms");

    let mut pointcloud_datatypes = HashSet::new();
    add_foxglove_schema(&mut pointcloud_datatypes, "foxglove.PointCloud");

    let mut laserscan_datatypes = HashSet::new();
    add_foxglove_schema(&mut laserscan_datatypes, "foxglove.LaserScan");

    let mut raw_image_datatypes = HashSet::new();
    add_foxglove_schema(&mut raw_image_datatypes, "foxglove.RawImage");

    let mut compressed_image_datatypes = HashSet::new();
    add_foxglove_schema(&mut compressed_image_datatypes, "foxglove.CompressedImage");

    let mut compressed_video_datatypes = HashSet::new();
    add_foxglove_schema(&mut compressed_video_datatypes, "foxglove.CompressedVideo");

    let mut camera_calibration_datatypes = HashSet::new();
    add_foxglove_schema(&mut camera_calibration_datatypes, "foxglove.CameraCalibration");

    let mut scene_update_datatypes = HashSet::new();
    add_foxglove_schema(&mut scene_update_datatypes, "foxglove.SceneUpdate");

    let mut pose_in_frame_datatypes = HashSet::new();
    add_foxglove_schema(&mut pose_in_frame_datatypes, "foxglove.PoseInFrame");

    let mut poses_in_frame_datatypes = HashSet::new();
    add_foxglove_schema(&mut poses_in_frame_datatypes, "foxglove.PosesInFrame");

    let mut grid_datatypes = HashSet::new();
    add_foxglove_schema(&mut grid_datatypes, "foxglove.Grid");

    let mut image_annotations_datatypes = HashSet::new();
    add_foxglove_schema(&mut image_annotations_datatypes, "foxglove.ImageAnnotations");
}
```