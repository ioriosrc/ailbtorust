```rust
use std::collections::HashMap;

use crate::{style::ObjectSummaryStyle};

/// Component that displays a summary for arrays and objects in the tree view.
pub fn ObjectSummary(props: &ObjectSummaryProps) -> Option<impl std::fmt::Display> {
    let value = &props.value;
    if !value.is_object() && !value.is_array() {
        return None;
    }

    let item_string = if value.is_array() {
        format!("{} items", value.len())
    } else {
        format!("{} keys", value.keys().count())
    };

    Some(get_item_string("", value, "", item_string))
}

pub struct ObjectSummaryProps {
    pub value: serde_json::Value,
}
```
Note: In Rust, `use std::collections::HashMap` is not required as Rust's standard library provides a more functional approach with types like `HashMap`. Additionally, Rust does not have the concept of null as it uses `Option<T>` where `T` can be any type, including `None`.