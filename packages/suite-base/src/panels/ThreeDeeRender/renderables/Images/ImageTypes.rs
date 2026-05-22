```rust
use std::time::{Duration, SystemTime};

use crate::{
    frame_ids::FrameId,
    headers::Header,
    messages::{CompressedImage, Image, RawImage},
};

/// NOTE: Remove this definition once it is available in @foxglove/schemas
pub type CompressedVideo = {
    timestamp: Duration;
    frame_id: FrameId;
    data: Vec<u8>;
    format: String,
};

pub type CompressedImageTypes = RosCompressedImage | CompressedImage;

/// NOTE: Remove this definition once it is available in @foxglove/schemas
pub type AnyImage = Image | RawImage | CompressedImage | CompressedVideo;

/// get the frame id from an image message
pub fn get_frame_id_from_image(image: &AnyImage) -> FrameId {
    match image {
        Image { header, .. } => FrameId::from(header.frame_id.clone()),
        _ => FrameId::from(image.header.frame_id),
    }
}

/// get the timestamp from an image message
pub fn get_timestamp_from_image(image: &AnyImage) -> SystemTime {
    match image {
        Image { header, .. } => header.stamp,
        CompressedImage { header, .. } => header.stamp,
        _ => image.timestamp,
    }
}
```