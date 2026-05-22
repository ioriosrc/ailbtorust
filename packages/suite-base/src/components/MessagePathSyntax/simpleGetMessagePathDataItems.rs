```rust
use lichtblick_message_path::{MessagePath, Immutable};
use lichtblick_suite_base::players::types::MessageEvent;
use lichtblick_suite_base::types::is TypedArray;

pub fn simple_get_message_path_data_items(
    message: Immutable<MessageEvent>,
    filled_in_path: Immutable<MessagePath>,
) -> Vec<impl std::any::Any> {
    // We don't care about messages that don't match the topic we're looking for.
    if message.topic() != filled_in_path.topic_name() {
        return vec![];
    }

    let mut results = Vec::new();

    fn traverse(value: &impl std::any::Any, path_index: usize) {
        let path_part = filled_in_path.message_path(path_index);
        if path_part.is_none() {
            results.push(value.clone());
            return;
        }
        if value.is_none() {
            return;
        }
        match path_part.as_ref().unwrap().type_() {
            "slice" => {
                if !value.is::<Vec<impl std::any::Any>>() && !is_typed_array(value) {
                    return;
                }
                if let Some((start, end)) = (&path_part.as_ref().unwrap()).get_start_end() {
                    for i in start..=end {
                        traverse(&value[i], path_index + 1);
                    }
                } else {
                    panic!("Variables in slices are not supported");
                }
            }
            "filter" => {
                if !filter_matches(path_part.as_ref().unwrap(), value) {
                    return;
                }
                traverse(value, path_index + 1);
            }
            "name" => {
                if let Some(name) = &path_part.as_ref().unwrap().get_name() {
                    if let Some(sub_value) = value.downcast_ref::<impl std::any::Any>() {
                        traverse(&sub_value[name], path_index + 1);
                    } else {
                        panic!("Name '{}' does not exist in the structure", name);
                    }
                } else {
                    panic!("Variable 'name' is not present in the path part");
                }
            }
            _ => unreachable!(),
        }
    }

    traverse(&message.message, 0);

    results
}
```