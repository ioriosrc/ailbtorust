```rust
use std::collections::HashMap;

fn normalize_annotations(input: HashMap<&str, serde_json::Value>, message_path: &str) -> Vec<serde_json::Value> {
    input.values().map(|annotation| {
        let mut normalized_annotation = serde_json::from_value(annotation.clone()).unwrap();

        // Normalize specific fields based on the message path
        match message_path {
            "foxglove.ImageAnnotations" => {
                if let Some(circles) = normalized_annotation.get_mut("circles") {
                    circles.iter_mut().for_each(|circle| {
                        circle["fill_color"] = serde_json::Value::from_iter(circle["fill_color"].as_object().unwrap().iter());
                        circle["outline_color"] = serde_json::Value::from_iter(circle["outline_color"].as_object().unwrap().iter());
                    });
                }
            }
            "foxglove.PointsAnnotation" => {
                if let Some(points) = normalized_annotation.get_mut("points") {
                    points.iter_mut().for_each(|point| {
                        point["outline_colors"] = serde_json::Value::from_iter(point["outline_colors"].as_array().unwrap().iter());
                        point["fill_color"] = serde_json::Value::from_iter(point["fill_color"].as_object().unwrap().iter());
                    });
                }
            }
            "foxglove.TextAnnotation" => {
                normalized_annotation["text_color"] = serde_json::Value::from_iter(normalized_annotation["text_color"].as_object().unwrap().iter());
            }
            _ => {}
        }

        normalized_annotation
    }).collect()
}
```