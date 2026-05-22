```rust
use crate::suite::{CameraInfo, ICameraModel};
use crate::types::CameraModelsMap;

pub fn select_camera_model(
  camera_info: CameraInfo,
  camera_models: CameraModelsMap,
) -> ICameraModel {
  match camera_models.get(&camera_info.distortion_model) {
    Some(camera_model) => camera_model.model_builder(camera_info),
    None => PinholeCameraModel::new(camera_info),
  }
}
```