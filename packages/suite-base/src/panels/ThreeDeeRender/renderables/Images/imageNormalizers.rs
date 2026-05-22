```rust
use crate::normalize_messages::*;

fn normalize_image_data(data: &[u8]) -> Vec<u8> {
    data.to_vec()
}

pub fn normalize_ros_image(message: &Message<RosImage>) -> RosImage {
    let header = message.header();
    let height = message.height.unwrap_or_default();
    let width = message.width.unwrap_or_default();
    let encoding = message.encoding.unwrap_or("");
    let is_bigendian = message.is_bigendian.unwrap_or(false);
    let step = message.step.unwrap_or(0);
    let data = normalize_image_data(&message.data());

    RosImage {
        header: normalize_header(header),
        height,
        width,
        encoding,
        is_bigendian,
        step,
        data,
    }
}

pub fn normalize_ros_compressed_image(
    message: &Message<RosCompressedImage>,
) -> RosCompressedImage {
    let header = message.header();
    let format = message.format.unwrap_or("");
    let data = normalize_byte_array(&message.data());

    RosCompressedImage {
        header: normalize_header(header),
        format,
        data,
    }
}

pub fn normalize_raw_image(message: &Message<RawImage>) -> RawImage {
    let timestamp = message.timestamp();
    let frame_id = message.frame_id().unwrap_or("");
    let height = message.height.unwrap_or_default();
    let width = message.width.unwrap_or_default();
    let encoding = message.encoding().unwrap_or("");
    let step = message.step.unwrap_or(0);
    let data = normalize_image_data(&message.data());

    RawImage {
        timestamp,
        frame_id,
        height,
        width,
        encoding,
        step,
        data,
    }
}

fn normalize_compressed_media<T: CompressedMedia>(
    message: &Message<T>,
) -> T {
    let header = message.header();
    let format = message.format().unwrap_or("");
    let data = normalize_byte_array(&message.data());

    T {
        timestamp: message.timestamp(),
        frame_id: message.frame_id().unwrap_or_default(),
        format,
        data,
    }
}

pub fn normalize_compressed_image(message: &Message<CompressedImage>) -> CompressedImage {
    normalize_compressed_media::<CompressedImage>(message)
}

pub fn normalize_compressed_video(message: &Message<CompressedVideo>) -> CompressedVideo {
    normalize_compressed_media::<CompressedVideo>(message)
}
```