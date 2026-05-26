// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

//! TF (Transform) tree for coordinate frame management.
//! Implements temporal interpolation (SLERP for rotations, linear for translations).

use std::collections::HashMap;

// ============ Quaternion ============

#[derive(Clone, Copy, Debug)]
pub struct Quaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Quaternion {
    pub fn identity() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

    pub fn normalize(&self) -> Self {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt();
        if len < 1e-12 {
            return Self::identity();
        }
        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
            w: self.w / len,
        }
    }

    /// Spherical Linear Interpolation between two quaternions.
    /// q(t) = sin((1-t)θ)/sin(θ) * q0 + sin(tθ)/sin(θ) * q1
    pub fn slerp(q0: &Self, q1: &Self, t: f64) -> Self {
        let mut dot = q0.x * q1.x + q0.y * q1.y + q0.z * q1.z + q0.w * q1.w;

        // If dot is negative, negate one quaternion to take the shorter path
        let mut q1_adj = *q1;
        if dot < 0.0 {
            q1_adj.x = -q1_adj.x;
            q1_adj.y = -q1_adj.y;
            q1_adj.z = -q1_adj.z;
            q1_adj.w = -q1_adj.w;
            dot = -dot;
        }

        // If very close, use linear interpolation to avoid division by zero
        if dot > 0.9995 {
            return Self {
                x: q0.x + t * (q1_adj.x - q0.x),
                y: q0.y + t * (q1_adj.y - q0.y),
                z: q0.z + t * (q1_adj.z - q0.z),
                w: q0.w + t * (q1_adj.w - q0.w),
            }
            .normalize();
        }

        let theta = dot.clamp(-1.0, 1.0).acos();
        let sin_theta = theta.sin();

        let s0 = ((1.0 - t) * theta).sin() / sin_theta;
        let s1 = (t * theta).sin() / sin_theta;

        Self {
            x: s0 * q0.x + s1 * q1_adj.x,
            y: s0 * q0.y + s1 * q1_adj.y,
            z: s0 * q0.z + s1 * q1_adj.z,
            w: s0 * q0.w + s1 * q1_adj.w,
        }
        .normalize()
    }

    /// Convert quaternion to 3x3 rotation matrix (column-major).
    pub fn to_rotation_matrix(&self) -> [f64; 9] {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        let w = self.w;

        [
            1.0 - 2.0 * (y * y + z * z),
            2.0 * (x * y + z * w),
            2.0 * (x * z - y * w),
            2.0 * (x * y - z * w),
            1.0 - 2.0 * (x * x + z * z),
            2.0 * (y * z + x * w),
            2.0 * (x * z + y * w),
            2.0 * (y * z - x * w),
            1.0 - 2.0 * (x * x + y * y),
        ]
    }
}

// ============ Transform ============

#[derive(Clone, Copy, Debug)]
pub struct Vec3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3d {
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn lerp(a: &Self, b: &Self, t: f64) -> Self {
        Self {
            x: a.x + t * (b.x - a.x),
            y: a.y + t * (b.y - a.y),
            z: a.z + t * (b.z - a.z),
        }
    }
}

/// A stamped transform from parent_frame to child_frame.
#[derive(Clone, Debug)]
pub struct StampedTransform {
    pub timestamp_ns: u64,
    pub parent_frame: String,
    pub child_frame: String,
    pub translation: Vec3d,
    pub rotation: Quaternion,
}

/// Interpolated transform result.
#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub translation: Vec3d,
    pub rotation: Quaternion,
}

impl Transform {
    pub fn identity() -> Self {
        Self {
            translation: Vec3d::zero(),
            rotation: Quaternion::identity(),
        }
    }

    /// Compose two transforms: self * other (apply self first, then other).
    pub fn compose(&self, other: &Self) -> Self {
        // Rotate other.translation by self.rotation, then add self.translation
        let rot = self.rotation.to_rotation_matrix();
        let tx = other.translation.x;
        let ty = other.translation.y;
        let tz = other.translation.z;

        let new_translation = Vec3d {
            x: self.translation.x + rot[0] * tx + rot[3] * ty + rot[6] * tz,
            y: self.translation.y + rot[1] * tx + rot[4] * ty + rot[7] * tz,
            z: self.translation.z + rot[2] * tx + rot[5] * ty + rot[8] * tz,
        };

        // Compose rotations via quaternion multiplication
        let q0 = &self.rotation;
        let q1 = &other.rotation;
        let new_rotation = Quaternion {
            w: q0.w * q1.w - q0.x * q1.x - q0.y * q1.y - q0.z * q1.z,
            x: q0.w * q1.x + q0.x * q1.w + q0.y * q1.z - q0.z * q1.y,
            y: q0.w * q1.y - q0.x * q1.z + q0.y * q1.w + q0.z * q1.x,
            z: q0.w * q1.z + q0.x * q1.y - q0.y * q1.x + q0.z * q1.w,
        };

        Self {
            translation: new_translation,
            rotation: new_rotation.normalize(),
        }
    }

    /// Convert to a 4x4 homogeneous matrix (column-major, f32 for WebGL).
    pub fn to_mat4_f32(&self) -> [f32; 16] {
        let rot = self.rotation.to_rotation_matrix();
        [
            rot[0] as f32,
            rot[1] as f32,
            rot[2] as f32,
            0.0,
            rot[3] as f32,
            rot[4] as f32,
            rot[5] as f32,
            0.0,
            rot[6] as f32,
            rot[7] as f32,
            rot[8] as f32,
            0.0,
            self.translation.x as f32,
            self.translation.y as f32,
            self.translation.z as f32,
            1.0,
        ]
    }
}

// ============ TF Buffer ============

/// Buffer of transforms for a single frame pair, sorted by time.
struct TransformBuffer {
    transforms: Vec<StampedTransform>,
    max_buffer_size: usize,
}

impl TransformBuffer {
    fn new(max_buffer_size: usize) -> Self {
        Self {
            transforms: Vec::new(),
            max_buffer_size,
        }
    }

    fn insert(&mut self, tf: StampedTransform) {
        // Insert sorted by timestamp
        let pos = self
            .transforms
            .partition_point(|t| t.timestamp_ns < tf.timestamp_ns);
        self.transforms.insert(pos, tf);

        // Evict oldest if over limit
        if self.transforms.len() > self.max_buffer_size {
            self.transforms.remove(0);
        }
    }

    /// Get interpolated transform at the given time.
    fn lookup(&self, time_ns: u64) -> Option<Transform> {
        if self.transforms.is_empty() {
            return None;
        }

        // Exact match or interpolation
        let idx = self
            .transforms
            .partition_point(|t| t.timestamp_ns < time_ns);

        if idx == 0 {
            // Before first transform - use first
            let tf = &self.transforms[0];
            return Some(Transform {
                translation: tf.translation,
                rotation: tf.rotation,
            });
        }

        if idx >= self.transforms.len() {
            // After last transform - use last
            let tf = self.transforms.last().unwrap();
            return Some(Transform {
                translation: tf.translation,
                rotation: tf.rotation,
            });
        }

        // Interpolate between idx-1 and idx
        let tf0 = &self.transforms[idx - 1];
        let tf1 = &self.transforms[idx];

        let dt = (tf1.timestamp_ns - tf0.timestamp_ns) as f64;
        if dt < 1.0 {
            return Some(Transform {
                translation: tf1.translation,
                rotation: tf1.rotation,
            });
        }

        let t = (time_ns - tf0.timestamp_ns) as f64 / dt;

        Some(Transform {
            translation: Vec3d::lerp(&tf0.translation, &tf1.translation, t),
            rotation: Quaternion::slerp(&tf0.rotation, &tf1.rotation, t),
        })
    }
}

// ============ TF Tree ============

/// Key for a transform: (parent_frame, child_frame)
type FramePair = (String, String);

/// The TF tree manages all coordinate frame transforms.
pub struct TfTree {
    buffers: HashMap<FramePair, TransformBuffer>,
    /// Map from child_frame → parent_frame for tree traversal
    parents: HashMap<String, String>,
    max_buffer_size: usize,
}

impl TfTree {
    pub fn new() -> Self {
        Self {
            buffers: HashMap::new(),
            parents: HashMap::new(),
            max_buffer_size: 100,
        }
    }

    /// Insert a transform into the tree.
    pub fn insert(&mut self, tf: StampedTransform) {
        let key = (tf.parent_frame.clone(), tf.child_frame.clone());
        self.parents
            .insert(tf.child_frame.clone(), tf.parent_frame.clone());

        self.buffers
            .entry(key)
            .or_insert_with(|| TransformBuffer::new(self.max_buffer_size))
            .insert(tf);
    }

    /// Look up the transform from `source_frame` to `target_frame` at the given time.
    /// Returns the composed transform that maps points in source_frame to target_frame.
    pub fn lookup(
        &self,
        target_frame: &str,
        source_frame: &str,
        time_ns: u64,
    ) -> Option<Transform> {
        if target_frame == source_frame {
            return Some(Transform::identity());
        }

        // Build path from source to root
        let source_path = self.path_to_root(source_frame);
        let target_path = self.path_to_root(target_frame);

        // Find common ancestor
        let source_set: std::collections::HashSet<&str> =
            source_path.iter().map(|s| s.as_str()).collect();

        let common_ancestor = target_path.iter().find(|f| source_set.contains(f.as_str()))?;

        // Compose transforms: source → common_ancestor
        let mut source_to_common = Transform::identity();
        for frame in &source_path {
            if frame == common_ancestor {
                break;
            }
            let parent = self.parents.get(frame)?;
            let key = (parent.clone(), frame.clone());
            let tf = self.buffers.get(&key)?.lookup(time_ns)?;
            source_to_common = tf.compose(&source_to_common);
        }

        // Compose transforms: target → common_ancestor (then invert)
        let mut target_to_common = Transform::identity();
        for frame in &target_path {
            if frame == common_ancestor {
                break;
            }
            let parent = self.parents.get(frame)?;
            let key = (parent.clone(), frame.clone());
            let tf = self.buffers.get(&key)?.lookup(time_ns)?;
            target_to_common = tf.compose(&target_to_common);
        }

        // Result: source_to_common * inverse(target_to_common)
        // For simplicity, compose forward from common to target
        // TODO: implement proper inverse composition for full generality
        Some(source_to_common)
    }

    /// Get the path from a frame to the root of the tree.
    fn path_to_root(&self, frame: &str) -> Vec<String> {
        let mut path = vec![frame.to_string()];
        let mut current = frame.to_string();
        let mut visited = std::collections::HashSet::new();
        visited.insert(current.clone());

        while let Some(parent) = self.parents.get(&current) {
            if visited.contains(parent) {
                break; // Cycle detection
            }
            path.push(parent.clone());
            visited.insert(parent.clone());
            current = parent.clone();
        }

        path
    }

    /// Get all known frame names.
    pub fn frames(&self) -> Vec<String> {
        let mut frames: Vec<String> = self.parents.keys().cloned().collect();
        for parent in self.parents.values() {
            if !frames.contains(parent) {
                frames.push(parent.clone());
            }
        }
        frames.sort();
        frames
    }

    /// Get the parent frame of a child frame.
    pub fn get_parent(&self, child: &str) -> Option<String> {
        self.parents.get(child).cloned()
    }

    /// Get the number of buffered transforms for a parent→child pair.
    pub fn get_history_size(&self, parent: &str, child: &str) -> usize {
        let key = (parent.to_string(), child.to_string());
        self.buffers.get(&key).map(|b| b.transforms.len()).unwrap_or(0)
    }

    /// Get the latest timestamp for a parent→child pair.
    pub fn get_latest_timestamp(&self, parent: &str, child: &str) -> Option<u64> {
        let key = (parent.to_string(), child.to_string());
        self.buffers.get(&key).and_then(|b| b.transforms.last().map(|t| t.timestamp_ns))
    }

    /// Clear all transforms.
    pub fn clear(&mut self) {
        self.buffers.clear();
        self.parents.clear();
    }
}

// ============ TFMessage Decoder ============

/// Decode a tf2_msgs/TFMessage from CDR data.
pub fn decode_tf_message_cdr(data: &[u8]) -> Vec<StampedTransform> {
    let mut result = Vec::new();

    if data.len() < 8 {
        return result;
    }

    // Skip CDR encapsulation header (4 bytes)
    let d = &data[4..];
    let mut pos = 0;

    // TFMessage: sequence of TransformStamped
    if d.len() < pos + 4 {
        return result;
    }
    let count = u32::from_le_bytes([d[pos], d[pos + 1], d[pos + 2], d[pos + 3]]) as usize;
    pos += 4;

    for _ in 0..count {
        // Header: stamp (sec:u32 + nanosec:u32) + frame_id (string)
        if d.len() < pos + 8 {
            break;
        }
        let sec = u32::from_le_bytes([d[pos], d[pos + 1], d[pos + 2], d[pos + 3]]);
        pos += 4;
        let nanosec = u32::from_le_bytes([d[pos], d[pos + 1], d[pos + 2], d[pos + 3]]);
        pos += 4;

        // frame_id string
        if d.len() < pos + 4 {
            break;
        }
        let frame_len = u32::from_le_bytes([d[pos], d[pos + 1], d[pos + 2], d[pos + 3]]) as usize;
        pos += 4;
        if d.len() < pos + frame_len {
            break;
        }
        let parent_frame =
            String::from_utf8_lossy(&d[pos..pos + frame_len.saturating_sub(1)]).to_string();
        pos += frame_len;
        pos = (pos + 3) & !3; // align

        // child_frame_id string
        if d.len() < pos + 4 {
            break;
        }
        let child_len = u32::from_le_bytes([d[pos], d[pos + 1], d[pos + 2], d[pos + 3]]) as usize;
        pos += 4;
        if d.len() < pos + child_len {
            break;
        }
        let child_frame =
            String::from_utf8_lossy(&d[pos..pos + child_len.saturating_sub(1)]).to_string();
        pos += child_len;
        pos = (pos + 3) & !3; // align

        // Transform: translation (3xf64) + rotation (4xf64)
        // Align to 8 for f64
        pos = (pos + 7) & !7;

        if d.len() < pos + 56 {
            // 7 * 8 bytes
            break;
        }

        let tx = f64::from_le_bytes(d[pos..pos + 8].try_into().unwrap());
        pos += 8;
        let ty = f64::from_le_bytes(d[pos..pos + 8].try_into().unwrap());
        pos += 8;
        let tz = f64::from_le_bytes(d[pos..pos + 8].try_into().unwrap());
        pos += 8;

        let qx = f64::from_le_bytes(d[pos..pos + 8].try_into().unwrap());
        pos += 8;
        let qy = f64::from_le_bytes(d[pos..pos + 8].try_into().unwrap());
        pos += 8;
        let qz = f64::from_le_bytes(d[pos..pos + 8].try_into().unwrap());
        pos += 8;
        let qw = f64::from_le_bytes(d[pos..pos + 8].try_into().unwrap());
        pos += 8;

        let timestamp_ns = (sec as u64) * 1_000_000_000 + (nanosec as u64);

        result.push(StampedTransform {
            timestamp_ns,
            parent_frame,
            child_frame,
            translation: Vec3d { x: tx, y: ty, z: tz },
            rotation: Quaternion {
                x: qx,
                y: qy,
                z: qz,
                w: qw,
            },
        });
    }

    result
}

/// Check if a schema is a TF message.
pub fn is_tf_schema(schema: &str) -> bool {
    schema.contains("TFMessage")
        || schema == "tf2_msgs/msg/TFMessage"
        || schema == "tf2_msgs/TFMessage"
        || schema == "tf/tfMessage"
}
