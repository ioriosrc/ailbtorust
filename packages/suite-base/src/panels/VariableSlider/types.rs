```rust
use std::any::{Any, TypeId};

#[derive(Debug)]
struct SliderProps {
    min: f64,
    max: f64,
    step: f64,
}

#[derive(Debug)]
struct VariableSliderConfig {
    slider_props: Option<SliderProps>,
    global_variable_name: String,
}
```