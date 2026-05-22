```rust
use test_case::test;
use wasm_bindgen_test::wasm_bindgen_test;

use crate::{diff_labels, DiffObject};
use crate::utils::*;

#[wasm_bindgen_test]
async fn diff_stats_without_id_label() {
    let data: DiffObject = serde_json::from_str(r#"{"ADDED": {"field1": "value1"}}"#).unwrap();
    mock_get_change_counts().await.expect("failed to get change counts");

    let actual = render_to_string::<DiffStats>(&data, "Item");
    assert!(actual.contains("+1"));
}

#[wasm_bindgen_test]
async fn diff_stats_with_id_label() {
    let data: DiffObject = serde_json::from_str(r#"{"ID": {"id": 123}}"#).unwrap();
    mock_get_change_counts().await.expect("failed to get change counts");

    let actual = render_to_string::<DiffStats>(&data, "Message");
    assert!(actual.contains("Message"));
}

#[wasm_bindgen_test]
async fn diff_stats_combined_scenarios() {
    let data: DiffObject = serde_json::from_str(r#"{"ID": {"uuid": 123}, "ADDED": {"newField": "value1"}, "DELETED": {"oldField": "value2"}, "CHANGED": "value2 => value1"}"#).unwrap();
    mock_get_change_counts().await.expect("failed to get change counts");

    let actual = render_to_string::<DiffStats>(&data, "Object");
    assert!(actual.contains("Object"));
}
```