```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Time {
    // Define the structure of a Time type here if needed
}

#[derive(Serialize, Deserialize)]
pub struct Color {
    // Define the structure of a Color type here if needed
}

#[derive(Serialize, Deserialize)]
pub struct Point2D {
    x: f64,
    y: f64,
}

pub type PathKey = String | i32;

#[derive(Serialize, Deserialize)]
pub struct CircleAnnotation {
    type_: String,
    stamp: Time,
    fill_color: Option<Color>,
    outline_color: Option<Color>,
    radius: f64,
    thickness: f64,
    position: Point2D,
    message_path: Vec<PathKey>,
}

#[derive(Serialize, Deserialize)]
pub struct PointsAnnotation {
    type_: String,
    stamp: Time,
    style: &'static str, // Use a string to represent the style
    points: Vec<Point2D>,
    outline_colors: Vec<Color>,
    outline_color: Option<Color>,
    thickness: f64,
    fill_color: Option<Color>,
    message_path: Vec<PathKey>,
}

#[derive(Serialize, Deserialize)]
pub struct TextAnnotation {
    type_: String,
    stamp: Time,
    position: Point2D,
    text: String,
    textColor: Color,
    background_color: Option<Color>,
    font_size: f64,
    padding: f64,
    message_path: Vec<PathKey>,
}

#[derive(Serialize, Deserialize)]
pub type Annotation = CircleAnnotation | PointsAnnotation | TextAnnotation;
```