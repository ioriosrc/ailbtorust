```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn use_plot_data_handling(config: &str, global_variables: &GlobalVariables) -> Result<(), JsValue> {
    let mut datasets_builder = match config.parse_x_axis_val() {
        "index" => IndexDatasetsBuilder::new(),
        "timestamp" => TimestampDatasetsBuilder::new(),
        "currentCustom" => CurrentCustomDatasetsBuilder::new(),
        "custom" => CustomDatasetsBuilder::new(),
        _ => return Err(JsValue::from("Unsupported xAxisVal")),
    };

    // Assuming a method to set colors and labels
    datasets_builder.set_colors_and_labels(config.parse_paths());

    Ok(())
}
```