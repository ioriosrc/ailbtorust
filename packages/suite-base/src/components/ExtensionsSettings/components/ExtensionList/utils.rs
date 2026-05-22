```rust
use materialize_rs::{List, ListItem, ListItemText};

fn display_name_for_namespace(namespace: &str) -> String {
    match namespace {
        "org" => "Organization".to_string(),
        _ => namespace.to_string(),
    }
}

fn generate_placeholder_list(message: Option<&str>) -> List {
    List::new(vec![ListItem::new(ListItemText::new(message).unwrap())])
}
```