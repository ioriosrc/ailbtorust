```rust
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Recent {
    id: String,
    source_id: String,
    title: String,
    type_: String,
    extra: HashMap<String, String>,
}

pub fn use_indexed_db_recents() -> (Vec<Recent>, impl Fn(&Recent) -> bool>) {
    // Simulate IndexedDB operations
    let recents = Vec::new();
    let has_recents = || !recents.is_empty();

    (recents.clone(), has_recents)
}
```