```rust
use std::collections::HashMap;

// Define the ImageModeConfig and DEFAULT_IMAGE_CONFIG as structs
struct ImageModeConfig {
    // Define the fields of the ImageModeConfig struct
    field1: i32,
    field2: f64,
    // Add more fields as needed
}

const DEFAULT_IMAGE_CONFIG: ImageModeConfig = ImageModeConfig {
    field1: 0,
    field2: 0.0,
    // Initialize other fields with default values
};

// Define the ImageModeEventMap as a HashMap
type ImageModeEventMap = HashMap<String, Box<dyn std::any>>;

// Define the ConfigWithDefaults struct as a union of ImageModeConfig and DEFAULT_IMAGE_CONFIG
type ConfigWithDefaults = ImageModeConfig | &'static str;
```