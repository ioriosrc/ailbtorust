```rust
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, MouseEvent};

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
  // Your Rust code here
  Ok(())
}

async fn find_canvas() -> CanvasRenderingContext2d {
  let canvas = document().query_selector("canvas")?;
  if canvas.is_null() {
    panic!("Could not find canvas element");
  }
  let ctx = canvas.get_context("2d").unwrap();
  ctx.expect("Could not get context")
}

#[wasm_bindgen]
pub async fn simulate_drag_click(
  point: [f64; 2] = [0.0, 0.0],
  canvas: CanvasRenderingContext2d = find_canvas(),
) -> Result<(), JsValue> {
  let client_x = point[0];
  let client_y = point[1];
  canvas.set_mousedown(client_x, client_y)?;
  await Promise.resolve();
  canvas.set_mouseup(client_x, client_y)?;

  Ok(())
}
```

Note: This code assumes that `document` and `canvas` are available in the context where this code is executed. Also, the use of `wasm_bindgen` for JavaScript interop is demonstrated, which might require additional configuration and setup based on your specific environment.