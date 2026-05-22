```rust
use std::f64;

pub type Quaternion = (f64, f64, f64, f64);

pub type Euler = (f64, f64, f64);

/**
 * Converts a quaternion to a Euler roll, pitch, yaw representation, in degrees.
 *
 * @param quaternion Input quaternion.
 * @returns Converted Euler angle roll, pitch, yaw representation, in degrees.
 */
pub fn quaternion_to_euler(quaternion: Quaternion) -> Euler {
  let (x, y, z, w) = quaternion;

  let to_degrees = f64::consts::PI / 180.0;
  let dcm00 = w * w + x * x - y * y - z * z;
  let dcm10 = 2.0 * (x * y + w * z);
  let dcm20 = 2.0 * (x * z - w * y);
  let dcm21 = 2.0 * (w * x + y * z);
  let dcm22 = w * w - x * x - y * y + z * z;
  let roll = to_degrees * f64::atan2(dcm21, dcm22);
  let pitch = to_degrees * f64::asin(-dcm20);
  let yaw = to_degrees * f64::atan2(dcm10, dcm00);

  (roll, pitch, yaw)
}
```