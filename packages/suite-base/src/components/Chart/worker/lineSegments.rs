```rust
use chart_js::core_3::model::PointElement;

type PointElementWithRawData = PointElement & {
  raw: {
    label_color: Option<String>;
  };
};

fn is_point_element_with_raw_data(element: &PointElement) -> bool {
  element.raw().contains_key("label_color")
}

/**
 * Returns the labelColor from the point, if available.
 */
pub fn line_segment_label_color(context: &ScriptableLineSegmentContext) -> Option<&str> {
  if is_point_element_with_raw_data(&context.p0) {
    context.p0.raw().get("label_color").and_then(|value| value.as_str())
  } else {
    None
  }
}
```