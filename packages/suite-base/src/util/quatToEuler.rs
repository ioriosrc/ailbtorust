```rust
use std::f64;

/// Convert a quaternion to roll-pitch-yaw Euler angles.
///
/// # Arguments
/// * `x` - The x component of the quaternion.
/// * `y` - The y component of the quaternion.
/// * `z` - The z component of the quaternion.
/// * `w` - The w component of the quaternion.
///
/// # Returns
/// A tuple containing the roll, pitch, and yaw angles in degrees.
pub fn quat_to_euler(x: f64, y: f64, z: f64, w: f64) -> (f64, f64, f64) {
    let mut temp_quaternion = [x, y, z, w];
    let mut temp_euler = [0.0; 3];

    // Calculate Euler angles from quaternion
    // Here we use the standard algorithm to convert a unit quaternion into Euler angles
    // https://en.wikipedia.org/wiki/Conversion_between_quaternions_and_Euler_angles

    // Extract components of the quaternion
    let (qx, qy, qz, qw) = temp_quaternion;

    // Calculate pitch
    let yaw = -(2.0 * qx * qz - 2.0 * qw * qy);
    let pitch = -(2.0 * qy * qz + 2.0 * qw * qx);
    let roll = (1.0 - 2.0 * qy * qy - 2.0 * qz * qz).acos();

    // Convert angles from radians to degrees
    let pitch_deg = pitch.to_radians();
    let yaw_deg = yaw.to_radians();
    let roll_deg = roll.to_radians();

    (roll_deg, pitch_deg, yaw_deg)
}
```