```rust
use std::vec::Vec;

type FloatArray = Vec<f64>;

type DistortionModel = &'static str;

#[derive(Debug)]
pub struct CameraInfo {
    width: usize,
    height: usize,
    binning_x: usize,
    binning_y: usize,
    roi: Roi,
    distortion_model: DistortionModel,
    D: FloatArray,
    K: FloatArray,
    P: FloatArray,
    R: FloatArray,
}

#[derive(Debug)]
struct Roi {
    x_offset: usize,
    y_offset: usize,
    height: usize,
    width: usize,
    do_rectify: bool,
}

pub trait ICameraModel {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn fx(&self) -> f64;
    fn fy(&self) -> f64;
    fn cx(&self) -> f64;
    fn cy(&self) -> f64;
    fn project_pixel_to_3d_plane(&self, out: &mut Vector3, pixel: &Roi);
    fn project_pixel_to_3d_ray(&self, out: &mut Vector3, pixel: &Roi);
}

pub type CameraModelBuilder = Fn(CameraInfo) -> ICameraModel;

#[derive(Debug)]
pub struct RegisterCameraModelArgs {
    name: DistortionModel,
    modelBuilder: CameraModelBuilder,
}
```