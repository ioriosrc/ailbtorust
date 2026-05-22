```rust
pub const GAUGE_CONFIG: &serde_json::Value = serde_json::json!({
  "messagePath": {
    "label": "Message path"
  },
  "minValue": {
    "label": "Min"
  },
  "maxValue": {
    "label": "Max"
  },
  "colorMode": {
    "label": "Color mode",
    "options": {
      "colorMap": "Color map",
      "gradient": "Gradient"
    }
  },
  "colorMap": {
    "label": "Color mode",
    "options": {
      "rainbow": "Rainbow",
      "redYellowGreen": "Red to green",
      "turbo": "Turbo"
    }
  },
  "reverse": {
    "label": "Reverse"
  }
});
```