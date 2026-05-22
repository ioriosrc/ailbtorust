// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

//! CDR and ROS1 message decoders for extracting image data and common field types.

/// Decoded compressed image data.
pub struct DecodedCompressedImage {
    pub format: String,
    pub data: Vec<u8>,
    pub frame_id: String,
}

/// Decoded raw image data.
pub struct DecodedRawImage {
    pub width: u32,
    pub height: u32,
    pub encoding: String,
    pub data: Vec<u8>,
    pub step: u32,
    pub frame_id: String,
}

/// CDR reader (little-endian).
struct CdrReader<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> CdrReader<'a> {
    fn new(data: &'a [u8]) -> Self {
        // Skip 4-byte CDR encapsulation header
        CdrReader { data, offset: 4 }
    }

    fn align(&mut self, n: usize) {
        let rem = self.offset % n;
        if rem != 0 {
            self.offset += n - rem;
        }
    }

    fn read_u8(&mut self) -> Option<u8> {
        if self.offset >= self.data.len() {
            return None;
        }
        let val = self.data[self.offset];
        self.offset += 1;
        Some(val)
    }

    fn read_u32(&mut self) -> Option<u32> {
        self.align(4);
        if self.offset + 4 > self.data.len() {
            return None;
        }
        let val = u32::from_le_bytes([
            self.data[self.offset],
            self.data[self.offset + 1],
            self.data[self.offset + 2],
            self.data[self.offset + 3],
        ]);
        self.offset += 4;
        Some(val)
    }

    fn read_i32(&mut self) -> Option<i32> {
        self.align(4);
        if self.offset + 4 > self.data.len() {
            return None;
        }
        let val = i32::from_le_bytes([
            self.data[self.offset],
            self.data[self.offset + 1],
            self.data[self.offset + 2],
            self.data[self.offset + 3],
        ]);
        self.offset += 4;
        Some(val)
    }

    fn read_f64(&mut self) -> Option<f64> {
        self.align(8);
        if self.offset + 8 > self.data.len() {
            return None;
        }
        let val = f64::from_le_bytes([
            self.data[self.offset],
            self.data[self.offset + 1],
            self.data[self.offset + 2],
            self.data[self.offset + 3],
            self.data[self.offset + 4],
            self.data[self.offset + 5],
            self.data[self.offset + 6],
            self.data[self.offset + 7],
        ]);
        self.offset += 8;
        Some(val)
    }

    fn read_string(&mut self) -> Option<String> {
        let len = self.read_u32()? as usize;
        if self.offset + len > self.data.len() || len == 0 {
            return None;
        }
        // CDR strings include null terminator in length
        let end = if self.data[self.offset + len - 1] == 0 {
            len - 1
        } else {
            len
        };
        let s = String::from_utf8_lossy(&self.data[self.offset..self.offset + end]).to_string();
        self.offset += len;
        Some(s)
    }

    fn read_byte_sequence(&mut self) -> Option<&'a [u8]> {
        let len = self.read_u32()? as usize;
        if self.offset + len > self.data.len() {
            return None;
        }
        let slice = &self.data[self.offset..self.offset + len];
        self.offset += len;
        Some(slice)
    }

    /// Skip ROS2 Header (stamp + frame_id), return frame_id.
    fn read_ros2_header(&mut self) -> Option<String> {
        // stamp.sec (int32)
        self.read_i32()?;
        // stamp.nanosec (uint32)
        self.read_u32()?;
        // frame_id (string)
        self.read_string()
    }
}

/// ROS1 reader (no alignment, no encapsulation).
struct Ros1Reader<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> Ros1Reader<'a> {
    fn new(data: &'a [u8]) -> Self {
        Ros1Reader { data, offset: 0 }
    }

    fn read_u32(&mut self) -> Option<u32> {
        if self.offset + 4 > self.data.len() {
            return None;
        }
        let val = u32::from_le_bytes([
            self.data[self.offset],
            self.data[self.offset + 1],
            self.data[self.offset + 2],
            self.data[self.offset + 3],
        ]);
        self.offset += 4;
        Some(val)
    }

    fn read_u8(&mut self) -> Option<u8> {
        if self.offset >= self.data.len() {
            return None;
        }
        let val = self.data[self.offset];
        self.offset += 1;
        Some(val)
    }

    fn read_string(&mut self) -> Option<String> {
        let len = self.read_u32()? as usize;
        if self.offset + len > self.data.len() {
            return None;
        }
        // ROS1 strings: length prefix (no null terminator)
        let s = String::from_utf8_lossy(&self.data[self.offset..self.offset + len]).to_string();
        self.offset += len;
        Some(s)
    }

    fn read_byte_sequence(&mut self) -> Option<&'a [u8]> {
        let len = self.read_u32()? as usize;
        if self.offset + len > self.data.len() {
            return None;
        }
        let slice = &self.data[self.offset..self.offset + len];
        self.offset += len;
        Some(slice)
    }

    /// Skip ROS1 Header (seq + stamp + frame_id), return frame_id.
    fn read_ros1_header(&mut self) -> Option<String> {
        // seq (uint32)
        self.read_u32()?;
        // stamp.secs (uint32)
        self.read_u32()?;
        // stamp.nsecs (uint32)
        self.read_u32()?;
        // frame_id (string)
        self.read_string()
    }
}

/// Decode a CompressedImage message from CDR (ROS2) encoding.
pub fn decode_compressed_image_cdr(data: &[u8]) -> Option<DecodedCompressedImage> {
    if data.len() < 8 {
        return None;
    }
    let mut reader = CdrReader::new(data);
    let frame_id = reader.read_ros2_header()?;
    let format = reader.read_string()?;
    let image_data = reader.read_byte_sequence()?;

    Some(DecodedCompressedImage {
        format,
        data: image_data.to_vec(),
        frame_id,
    })
}

/// Decode a CompressedImage message from ros1msg encoding.
pub fn decode_compressed_image_ros1(data: &[u8]) -> Option<DecodedCompressedImage> {
    if data.len() < 16 {
        return None;
    }
    let mut reader = Ros1Reader::new(data);
    let frame_id = reader.read_ros1_header()?;
    let format = reader.read_string()?;
    let image_data = reader.read_byte_sequence()?;

    Some(DecodedCompressedImage {
        format,
        data: image_data.to_vec(),
        frame_id,
    })
}

/// Decode a raw Image message from CDR (ROS2) encoding.
pub fn decode_raw_image_cdr(data: &[u8]) -> Option<DecodedRawImage> {
    if data.len() < 20 {
        return None;
    }
    let mut reader = CdrReader::new(data);
    let frame_id = reader.read_ros2_header()?;
    let height = reader.read_u32()?;
    let width = reader.read_u32()?;
    let encoding = reader.read_string()?;
    let _is_bigendian = reader.read_u8()?;
    let step = reader.read_u32()?;
    let image_data = reader.read_byte_sequence()?;

    Some(DecodedRawImage {
        width,
        height,
        encoding,
        data: image_data.to_vec(),
        step,
        frame_id,
    })
}

/// Decode a raw Image message from ros1msg encoding.
pub fn decode_raw_image_ros1(data: &[u8]) -> Option<DecodedRawImage> {
    if data.len() < 20 {
        return None;
    }
    let mut reader = Ros1Reader::new(data);
    let frame_id = reader.read_ros1_header()?;
    let height = reader.read_u32()?;
    let width = reader.read_u32()?;
    let encoding = reader.read_string()?;
    let _is_bigendian = reader.read_u8()?;
    let step = reader.read_u32()?;
    let image_data = reader.read_byte_sequence()?;

    Some(DecodedRawImage {
        width,
        height,
        encoding,
        data: image_data.to_vec(),
        step,
        frame_id,
    })
}

/// Try to decode a CompressedImage from any encoding.
pub fn decode_compressed_image(data: &[u8], msg_encoding: &str) -> Option<DecodedCompressedImage> {
    match msg_encoding {
        "cdr" => decode_compressed_image_cdr(data),
        "ros1" | "ros1msg" => decode_compressed_image_ros1(data),
        _ => {
            // Try CDR first, then ROS1
            decode_compressed_image_cdr(data).or_else(|| decode_compressed_image_ros1(data))
        }
    }
}

/// Try to decode a raw Image from any encoding.
pub fn decode_raw_image(data: &[u8], msg_encoding: &str) -> Option<DecodedRawImage> {
    match msg_encoding {
        "cdr" => decode_raw_image_cdr(data),
        "ros1" | "ros1msg" => decode_raw_image_ros1(data),
        _ => decode_raw_image_cdr(data).or_else(|| decode_raw_image_ros1(data)),
    }
}

/// Decode a numeric value from a simple message (for plot panel).
/// Tries to extract the first f64 from the message data.
pub fn decode_numeric_cdr(data: &[u8]) -> Option<f64> {
    if data.len() < 12 {
        return None;
    }
    let mut reader = CdrReader::new(data);
    // Try reading as Float64 (std_msgs/Float64: data field is f64)
    reader.read_f64()
}

/// Check if a schema name is a compressed image type.
pub fn is_compressed_image_schema(schema: &str) -> bool {
    schema.contains("CompressedImage")
        || schema.contains("compressed_image")
        || schema == "foxglove.CompressedImage"
}

/// Check if a schema name is a raw image type.
pub fn is_raw_image_schema(schema: &str) -> bool {
    (schema.contains("Image") && !schema.contains("Compressed"))
        || schema == "sensor_msgs/Image"
        || schema == "sensor_msgs/msg/Image"
        || schema == "foxglove.RawImage"
}

/// Check if a schema name is suitable for 3D visualization.
pub fn is_3d_schema(schema: &str) -> bool {
    schema.contains("PointCloud2")
        || schema.contains("Marker")
        || schema.contains("LaserScan")
        || schema.contains("Odometry")
        || schema.contains("Path")
        || schema.contains("Pose")
        || schema.contains("OccupancyGrid")
        || schema.contains("Grid")
        || schema.contains("Transform")
        || schema.contains("SceneUpdate")
}
