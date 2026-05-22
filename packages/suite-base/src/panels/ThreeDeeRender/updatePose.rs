```rust
use three::{Object3D, Position, Quaternion};

fn update_pose(
    renderable: &mut Object3D,
    transform_tree: &TransformTree,
    render_frame_id: u64,
    fixed_frame_id: u64,
    src_frame_id: &str,
    dst_time: f64,
    src_time: f64,
) -> bool {
    if let Some(ref mut pose) = renderable.user_data.get::<Option<Pose>>().unwrap_or(&None) {
        let pose_applied = transform_tree.apply(temp_pose(), pose, render_frame_id, fixed_frame_id, src_frame_id, dst_time, src_time);
        renderable.visible = pose_applied;
        if pose_applied {
            let p = &pose.position;
            let q = &pose.orientation;
            renderable.set_position(p.x as f32, p.y as f32, p.z as f32);
            renderable.set_rotation(q.x as f32, q.y as f32, q.z as f32, q.w as f32);
        }
    } else {
        println!("Missing userData.pose for {}", renderable.name);
        return false;
    }
    pose_applied
}
```