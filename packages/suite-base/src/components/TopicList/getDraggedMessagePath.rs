```rust
use std::str;

pub fn get_dragged_message_path(tree_item: TopicListItem) -> DraggedMessagePath {
    match tree_item.type_ {
        "topic" => {
            let path = quote_topic_name_if_needed(&tree_item.item.item.name);
            let root_schema_name = &tree_item.item.item.schema_name;
            DraggedMessagePath {
                path,
                root_schema_name: *root_schema_name,
                is_topic: true,
                is_leaf: false,
                topic_name: tree_item.item.item.name.to_string(),
            }
        },
        "schema" => {
            let path = &tree_item.item.item.full_path;
            let root_schema_name = &tree_item.item.item.topic.schema_name;
            let suffix_is_leaf = tree_item.item.item.suffix.is_leaf();
            DraggedMessagePath {
                path: *path,
                root_schema_name: *root_schema_name,
                is_topic: false,
                is_leaf: suffix_is_leaf,
                topic_name: tree_item.item.item.topic.name.to_string(),
            }
        },
    }
}
```