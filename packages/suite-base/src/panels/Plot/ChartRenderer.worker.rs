```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ChartRenderer {
  chart: Chart,
}

impl ChartRenderer {
  pub async fn new(canvas: OffscreenCanvas, device_pixel_ratio: f64, grid_color: &str, tick_color: &str) -> Self {
    let options = ChartOptions::new();
    options.set_grid_line_color(grid_color);
    options.set_tick_color(tick_color);

    let chart = Chart::create(
      canvas,
      &options,
      Some(Box::new(LineController)),
    );

    Self { chart }
  }

  pub fn render(&mut self) {
    // Implement rendering logic here
  }
}

#[wasm_bindgen]
pub struct ChartOptions {
  grid_line_color: Option<&str>,
  tick_color: Option<&str>,
}

impl ChartOptions {
  pub fn new() -> Self {
    Self {
      grid_line_color: None,
      tick_color: None,
    }
  }

  pub fn set_grid_line_color(&mut self, color: &str) {
    self.grid_line_color = Some(color);
  }

  pub fn set_tick_color(&mut self, color: &str) {
    self.tick_color = Some(color);
  }
}
```