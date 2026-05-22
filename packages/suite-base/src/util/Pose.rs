```rust
use crate::messages::MutablePose; // Assuming the correct module is used

// Create a new empty pose object
pub fn empty_pose() -> MutablePose {
  MutablePose {
    position: Position { x: 0.0, y: 0.0, z: 0.0 },
    orientation: Orientation { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
  }
}
```