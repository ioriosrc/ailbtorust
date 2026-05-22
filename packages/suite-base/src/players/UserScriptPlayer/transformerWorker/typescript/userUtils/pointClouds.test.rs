```rust
use std::ops::{Add, Sub};

pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub fn norm(point: &Point) -> f64 {
    (point.x.powi(2) + point.y.powi(2) + point.z.powi(2)).sqrt()
}

pub fn set_ray_distance(point: Point, distance: f64) -> Point {
    let x = point.x * distance;
    let y = point.y * distance;
    let z = point.z * distance;
    Point { x, y, z }
}

pub struct RangeView<Point> {
    points: Vec<Point>,
    range: f64,
}

impl<Point> RangeView<Point> {
    pub fn new(points: Vec<Point>, range: f64) -> Self {
        self.points = points;
        self.range = range;
    }

    // Implement methods to manipulate the RangeView as needed
}
```