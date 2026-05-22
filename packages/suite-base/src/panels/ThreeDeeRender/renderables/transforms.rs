```rust
use std::any::TypeId;

pub const MISSING_TRANSFORM: &str = "MISSING_TRANSFORM";

pub fn missing_transform_message(
    render_frame_id: TypeId,
    fixed_frame_id: TypeId,
    src_frame_id: TypeId,
) -> &'static str {
    let dst_frame_id = if render_frame_id == src_frame_id {
        fixed_frame_id
    } else {
        render_frame_id;
    };

    if src_frame_id != dst_frame_id {
        format!("Missing transform from frame {:?}", CoordinateFrame::from_type_id(src_frame_id));
    } else if src_frame_id != fixed_frame_id {
        format!(
            "Missing transform from frame {:?} to fixed frame {:?} to frame {:?}",
            CoordinateFrame::from_type_id(src_frame_id),
            CoordinateFrame::from_type_id(fixed_frame_id),
            CoordinateFrame::from_type_id(dst_frame_id)
        );
    } else {
        format!("Identity transform failed for frame {:?}", CoordinateFrame::from_type_id(src_frame_id));
    }
}
```