```rust
use std::collections::HashMap;

type StructuredItemsState = HashMap<String, MessagePathStructureItem>;

fn create_structure_items_by_path_store() -> impl FnOnce(StructuredItemsState) {
    move |state| {
        let mut structured_items_map: HashMap<String, MessagePathStructureItem> = state.clone();
        move || structured_items_map
    }
}
```