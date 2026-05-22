```rust
use serde_json::{Map, Value};
use std::convert::TryFrom;

// Define the mock types
struct MockTime {
    sec: i32,
    nsec: i64,
}

struct MockHeader {
    stamp: MockTime,
    frame_id: String,
}

#[derive(Debug)]
struct RosImage {
    header: MockHeader,
    height: u32,
    width: u32,
    encoding: String,
    is_bigendian: bool,
    step: usize,
    data: Vec<u8>,
}

#[derive(Debug)]
struct CompressedImage {
    timestamp: MockTime,
    frame_id: String,
    format: String,
    data: Vec<u8>,
}

#[derive(Debug)]
struct RawImage {
    timestamp: MockTime,
    frame_id: String,
    height: u32,
    width: u32,
    encoding: String,
    step: usize,
    data: Vec<u8>,
}

// Define the normalization functions
fn normalize_ros_image(input: &RosImage) -> RosImage {
    // Implementation of normalizeRosImage
    input.clone()
}

fn normalize_ros_compressed_image(input: &RosCompressedImage) -> RosCompressedImage {
    // Implementation of normalizeRosCompressedImage
    input.clone()
}

fn normalize_raw_image(input: &RawImage) -> RawImage {
    // Implementation of normalizeRawImage
    input.clone()
}

fn normalize_compressed_image(input: &CompressedImage) -> CompressedImage {
    // Implementation of normalizeCompressedImage by calling normalizeRosCompressedImage
    normalize_ros_compressed_image(input)
}

// Define the test cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_ros_image() {
        let mock_time = MockTime { sec: 123, nsec: 456000000 };
        let mock_header = MockHeader { stamp: mock_time.clone(), frame_id: "test_frame".to_string() };
        let mock_data = vec![1, 2, 3, 4];
        let input: RosImage = RosImage {
            header: mock_header,
            height: 480,
            width: 640,
            encoding: String::from("rgb8"),
            is_bigendian: false,
            step: 1920,
            data: mock_data.clone(),
        };
        let expected = RosImage {
            header: mock_header.clone(),
            height: 480,
            width: 640,
            encoding: String::from("rgb8"),
            is_bigendian: false,
            step: 1920,
            data: mock_data.clone(),
        };
        assert_eq!(normalize_ros_image(&input), expected);
    }

    // Add more test cases as needed
}
```