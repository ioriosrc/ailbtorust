```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn message_path_row(
  message_path_result: &JsValue,
  style: JsValue,
  selected: bool,
  onClick: Closure<dyn FnMut()>,
  onContextMenu: Closure<dyn FnMut()>,
) -> JsValue {
  let cx = js_sys::get_global_object().unwrap();
  let classes = cx
    .get("classes")
    .expect("Failed to get 'classes' property");

  let item: JsValue =JsValue::from({
    "path": message_path_result.get("fullPath").unwrap(),
    "rootSchemaName": message_path_result.get("topicSchemaName").unwrap(),
    "isTopic": false,
    "isLeaf": message_path_result.get("isLeaf").unwrap(),
    "topicName": message_path_result.get("topic.name").unwrap(),
  });

  let { connect_drag_source, connect_drag_preview, cursor, is_dragging, dragged_item_count } = js_sys::get_global_object().unwrap();

  let combined_ref: JsValue = Closure::new(move |el| {
    connect_drag_source(&el);
    connect_drag_preview(&el);
  });

  let style = style.as_object().unwrap();
  let cursor = cursor.as_string().unwrap();

  Ok(JsValue::from({
    "className": cx.get("classes").expect("Failed to get 'classes' property"),
    "style": JsValue::from(style),
    "cursor": JsValue::from(cursor),
    "onClick": onClick.as_ref(),
    "onContextMenu": onContextMenu.as_ref(),
  }))
}
```