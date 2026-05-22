```rust
use rand::Rng; // Import necessary for random number generation

const INITIAL_BRIGHTNESS: f32 = 50.0;
const INITIAL_CONTRAST: f32 = 70.0;
const LOWER_BRIGHTNESS_LIMIT: f32 = 0.1;
const LOWER_CONTRAST_LIMIT: f32 = 0.1;
const MAX_BRIGHTNESS: f32 = 95.0;
const MAX_CONTRAST: f32 = 95.0;
const MIN_BRIGHTNESS: f32 = 10.0;
const MIN_CONTRAST: f32 = 10.0;
const UPPER_BRIGHTNESS_LIMIT: f32 = 99.9;
const UPPER_CONTRAST_LIMIT: f32 = 99.9;

fn clamp_brightness(brightness: f32) -> f32 {
    if brightness < LOWER_BRIGHTNESS_LIMIT {
        LOWER_BRIGHTNESS_LIMIT
    } else if brightness > UPPER_BRIGHTNESS_LIMIT {
        UPPER_BRIGHTNESS_LIMIT
    } else {
        brightness
    }
}

fn clamp_contrast(contrast: f32) -> f32 {
    if contrast < LOWER_CONTRAST_LIMIT {
        LOWER_CONTRAST_LIMIT
    } else if contrast > UPPER_CONTRAST_LIMIT {
        UPPER_CONTRAST_LIMIT
    } else {
        contrast
    }
}

// Helper function to generate a random number within the specified range
fn basic_builder_number(min: f32, max: f32) -> f32 {
    let rng = rand::thread_rng();
    rng.gen_range(min..=max)
}
```