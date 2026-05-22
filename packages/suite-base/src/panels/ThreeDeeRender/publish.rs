```rust
use std::collections::HashMap;
use chrono::Utc; // Assuming Utc is a valid date time library in Rust

pub struct Point {
    x: f64,
    y: f64,
}

pub struct Pose {
    position: Point,
    orientation: Vec3, // Assuming Vec3 is a 3D vector structure
}

// Implementing the required methods for Vec3 if necessary
struct Vec3;

impl Pose {
    fn new(position: Point, orientation: Vec3) -> Self {
        Self { position, orientation }
    }

    pub fn covariance(&self) -> CovarianceArray {
        // Assuming CovarianceArray is a struct that can be constructed from two vectors
        let x_dev = 1.0; // Example deviation for x coordinate
        let y_dev = 2.0; // Example deviation for y coordinate
        let theta_dev = 3.0; // Example deviation for orientation angle

        CovarianceArray {
            positions: vec![x_dev, 0.0, 0.0],
            orientations: vec![-y_dev, x_dev, 0.0],
        }
    }
}

pub struct PointStamped {
    point: Point,
    timestamp: Utc::now(),
}

pub struct PoseStamped {
    pose: Pose,
    timestamp: Utc::now(),
}

// Assuming CovarianceArray is a struct that can be constructed from two vectors
struct CovarianceArray {
    positions: Vec<f64>,
    orientations: Vec<f64>,
}

fn makePointMessage(point: Point, frame_id: &str) -> (PoseStamped, String) {
    let time = Utc::now();
    let pose_stamped = PoseStamped {
        point: PointStamped {
            point,
            timestamp,
        },
        frame_id: frame_id.to_string(),
    };

    (pose_stamped, "PointStamped".to_string())
}

fn makePoseMessage(pose: Pose, frame_id: &str) -> (PoseStamped, String) {
    let time = Utc::now();
    let pose_stamped = PoseStamped {
        pose,
        timestamp,
    };

    (pose_stamped, "PoseStamped".to_string())
}

fn makePoseEstimateMessage(pose: Pose, frame_id: &str, x_dev: f64, y_dev: f64, theta_dev: f64) -> (PoseWithCovarianceStamped, String) {
    let time = Utc::now();
    let pose_with_covariance_stamped = PoseWithCovarianceStamped {
        pose,
        covariance: makeCovarianceArray(x_dev, y_dev, theta_dev),
        header: Header {
            stamp: time.to_datetime(),
            frame_id: frame_id.to_string(),
        },
    };

    (pose_with_covariance_stamped, "PoseWithCovarianceStamped".to_string())
}

// Assuming Header is a struct with required fields like stamp and frame_id
struct Header {
    stamp: chrono::DateTime<Utc>,
    frame_id: String,
}
```