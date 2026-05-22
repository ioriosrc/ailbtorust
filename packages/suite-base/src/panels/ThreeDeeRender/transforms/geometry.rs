```rust
use glam::Vec3;

pub type Point = Vec3<f32>;
pub type Orientation = Quat;
pub type Pose = (Point, Orientation);

fn make_pose() -> Pose {
  (Vec3::ZERO, Quat::IDENTITY)
}

fn xyzrpy_to_pose(xyz: Vec3, rpy: Vec3) -> Pose {
  let o = Quat::from_euler([0.0, 0.0, 0.0], rpy.x, rpy.y, rpy.z);
  (xyz, o)
}

// Helper functions for constructing geometry primitives that can be used with
// gl-matrix. These methods are preferred over the gl-matrix equivalents since
// they produce number[] arrays instead of Float32Array, which have less
// precision and are slower (float32 requires upcasting/downcasting to do math
// in JavaScript).

fn vec3_identity() -> Vec3<f32> {
  [0.0, 0.0, 0.0]
}

fn vec3_from_values(x: f32, y: f32, z: f32) -> Vec3<f32> {
  [x, y, z]
}

fn vec3_clone(a: Vec3<f32>) -> Vec3<f32> {
  a
}

fn quat_identity() -> Quat {
  Quat::identity()
}

fn quat_from_values(x: f32, y: f32, z: f32, w: f32) -> Quat {
  Quat::from_xyzw(x, y, z, w)
}

fn quat_clone(q: Quat) -> Quat {
  q
}

fn mat4_identity() -> Mat4<f32> {
  [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]
}

fn mat4_from_values(
  m00: f32,
  m01: f32,
  m02: f32,
  m03: f32,
  m10: f32,
  m11: f32,
  m12: f32,
  m13: f32,
  m20: f32,
  m21: f32,
  m22: f32,
  m23: f32,
  m30: f32,
  m31: f32,
  m32: f32,
  m33: f32,
) -> Mat4<f32> {
  [m00, m01, m02, m03, m10, m11, m12, m13, m20, m21, m22, m23, m30, m31, m32, m33]
}

fn mat4_clone(m: Mat4<f32>) -> Mat4<f32> {
  [m[0], m[1], m[2], m[3], m[4], m[5], m[6], m[7], m[8], m[9], m[10], m[11], m[12], m[13], m[14], m[15]]
}

/**
 * Test if two numbers are approximately equal.
 */
pub fn approx_eq(v1: f32, v2: f32, epsilon: f32) -> bool {
  (v1 - v2).abs() <= epsilon
}

/**
 * Test if two quaternions are approximately equal.
 */
pub fn quat_aapproxEq(q1: Quat, q2: Quat) -> bool {
  approx_eq(q1.x, q2.x, epsilon) && approx_eq(q1.y, q2.y, epsilon) && approx_eq(q1.z, q2.z, epsilon) && approx_eq(q1.w, q2.w, epsilon)
}

/**
 * Test if two poses are approximately equal.
 */
pub fn pose_approxEq(p1: Pose, p2: Pose) -> bool {
  approx_eq(p1.0.x, p2.0.x, epsilon) &&
    approx_eq(p1.0.y, p2.0.y, epsilon) &&
    approx_eq(p1.0.z, p2.0.z, epsilon) &&
    quat_aapproxEq(p1.1, p2.1)
}

/**
 * Returns a quaternion representing the rotational component of a
 * transformation matrix. The matrix must not have any scaling applied to it.
 * @param out Quaternion to receive the rotation component
 * @param mat Matrix to be decomposed (input)
 * @param scaling Scaling of the matrix (input)
 * @return out
 */
pub fn get_rotation_no_scaling(out: Quat, mat: Mat4<f32>) -> Quat {
  let m11 = mat.m00;
  let m12 = mat.m01;
  let m13 = mat.m02;
  let m21 = mat.m04;
  let m22 = mat.m05;
  let m23 = mat.m06;
  let m31 = mat.m08;
  let m32 = mat.m09;
  let m33 = mat.m10;
  let trace = m11 + m22 + m33;
  let S = 0.0;
  if (trace > 0.0) {
    S = (1.0 + trace).sqrt() * 2.0;
    out.x = (m23 - m32) / S;
    out.y = (m31 - m13) / S;
    out.z = (m12 - m21) / S;
    out.w = 0.25 * S;
  } else if (m11 > m22 && m11 > m33) {
    S = (1.0 + m11 - m22 - m33).sqrt() * 2.0;
    out.x = 0.25 * S;
    out.y = (m12 + m21) / S;
    out.z = (m31 + m13) / S;
    out.w = (m23 - m32) / S;
  } else if (m22 > m33) {
    S = (1.0 + m22 - m11 - m33).sqrt() * 2.0;
    out.x = (m31 - m13) / S;
    out.y = 0.25 * S;
    out.z = (m23 + m32) / S;
    out.w = (m12 - m21) / S;
  } else {
    S = (1.0 + m33 - m11 - m22).sqrt() * 2.0;
    out.x = (m12 - m21) / S;
    out.y = (m31 + m13) / S;
    out.z = 0.25 * S;
    out.w = (m23 - m32) / S;
  }
}
```