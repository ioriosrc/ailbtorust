```rust
use num_traits::Float;

type Point = (f32, f32, f32); // Assuming float32 for simplicity, can be changed to float64
type Quaternion = (f32, f32, f32, f32);
type Vector3 = (f32, f32, f32);

fn euler_to_quaternion(rpy: Vector3) -> Quaternion {
    let (roll, pitch, yaw) = rpy;

    let cy = F::cos(yaw * 0.5);
    let sy = F::sin(yaw * 0.5);
    let cr = F::cos(roll * 0.5);
    let sr = F::sin(roll * 0.5);
    let cp = F::cos(pitch * 0.5);
    let sp = F::sin(pitch * 0.5);

    let w = cy * cr * cp + sy * sr * sp;
    let x = cy * sr * cp - sy * cr * sp;
    let y = cy * cr * sp + sy * sr * cp;
    let z = sy * cr * cp - cy * sr * sp;

    (x, y, z, w)
}

fn make_covariance_array(x_dev: f32, y_dev: f32, theta_dev: f32) -> Vec<f32> {
    let mut covariance = vec![0.0; 36];

    // Assuming each element is assigned the same value for simplicity
    for i in 0..18 {
        covariance[i * 6] = x_dev.powi(2);
        covariance[i * 6 + 1] = y_dev.powi(2);
        covariance[i * 6 + 5] = theta_dev.powi(2);
    }

    covariance
}
```