```rust
use react_mosaic_component::{get_node_at_path, MosaicKey, MosaicRootActions};

fn get_panel_type_from_mosaic(
    mosaic_window_actions: Option<&MosaicWindowActions>,
    mosaic_actions: Option<&MosaicRootActions<MosaicKey>>,
) -> Option<String> {
    if mosaic_window_actions.is_none() || mosaic_actions.is_none() {
        return None;
    }
    let root = mosaic_actions.as_ref().unwrap().get_root();
    let node = get_node_at_path(root, mosaic_window_actions.as_ref().unwrap().path()).unwrap();

    if !node.is_string() {
        return Some("Non-leaf node".to_string());
    }

    let type_ = get_panel_type_from_id(node);

    type_
}
```