```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn create_renderer(
  canvas_div: &web_sys::HtmlElement,
  theme: &web_sys::Theme,
) -> Result<OffscreenCanvasRenderer, JsValue> {
  let client_rect = canvas_div.get_bounding_client_rect();
  let canvas = document.createElement("canvas")?;
  canvas.style.width = "100%";
  canvas.style.height = "100%";
  canvas.style.position = "absolute";
  canvas.set_attribute("width", &client_rect.width.to_string());
  canvas.set_attribute("height", &client_rect.height.to_string());

  canvas_div.append_child(&canvas)?;

  let offscreen_canvas = canvas.transfer_control_to_offscreen()?;
  let new_renderer = OffscreenCanvasRenderer::new(offscreen_canvas, theme);

  Ok(new_renderer)
}
```