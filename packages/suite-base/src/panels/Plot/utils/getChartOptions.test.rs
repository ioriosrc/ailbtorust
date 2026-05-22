```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct ChartOptions {
    maintain_aspect_ratio: bool,
    animation: bool,
    elements: serde_json::Value,
    interaction: serde_json::Value,
    device_pixel_ratio: f64,
    font: serde_json::Value,
    responsive: bool,
    scales: serde_json::Value,
    plugins: serde_json::Value,
}

fn get_chart_options(options: &ChartOptions) -> ChartOptions {
    options.clone()
}
```