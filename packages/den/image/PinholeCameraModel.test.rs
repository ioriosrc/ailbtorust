```rust
use std::vec::Vec;

#[derive(Debug)]
struct PinholeCameraModel {
    fx: f64,
    fy: f64,
    cx: f64,
    cy: f64,
    K: Vec<f64>,
    R: Vec<f64>,
    P: Vec<f64>,
}

impl PinholeCameraModel {
    fn new(width: u32, height: u32, fov: f64) -> Self {
        let cx = width / 2.0;
        let cy = height / 2.0;
        let fx = width as f64 / (2.0 * (fov * std::f64::consts::PI / 180.0).tan());
        let fy = fx;

        Self {
            K: vec![fx, 0.0, cx, 0.0, fy, cy, 0.0, 0.0, 1.0],
            R: vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
            P: vec![fx, 0.0, cx, 0.0, 0.0, fx, cy, 0.0, 0.0, 0.0, 1.0],
            width,
            height,
        }
    }

    fn project_pixel_to_3d_plane(&self, point: &mut Vec<f64>, pixel: &Vec<f64>) {
        let x = pixel[0];
        let y = pixel[1];

        // Convert to normalized image coordinates
        let u = (x as f64 - self.cx) / self.fx;
        let v = (y as f64 - self.cy) / self.fy;

        // Project 2D pixel to 3D point
        let z = 1.0; // Assuming the camera is at infinity, or some depth value
        point[0] = u * z;
        point[1] = v * z;
    }

    fn project_pixel_to_3d_ray(&self, ray: &mut Vec<f64>, pixel: &Vec<f64>) {
        let x = pixel[0];
        let y = pixel[1];

        // Convert to normalized image coordinates
        let u = (x as f64 - self.cx) / self.fx;
        let v = (y as f64 - self.cy) / self.fy;

        // Project 2D pixel to 3D ray
        let z = 1.0; // Assuming the camera is at infinity, or some depth value
        ray[0] = u * z;
        ray[1] = v * z;
    }

    fn undistort_pixel(&self, rectified: &mut Vec<f64>, pixel: &Vec<f64>) -> bool {
        // Placeholder for the actual undistortion logic
        rectified.copy_from(pixel);
        true
    }

    fn distort_pixel(&self, unrectified: &mut Vec<f64>, rectified: &Vec<f64>) -> bool {
        // Placeholder for the actual distortion logic
        unrectified.copy_from(rectified);
        true
    }
}
```