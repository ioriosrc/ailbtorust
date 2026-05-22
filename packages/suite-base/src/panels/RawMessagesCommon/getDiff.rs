```rust
use std::collections::{HashMap, HashSet};

use crate::{
    common::MessagePathSyntax::{isTypicalFilterName},
    panels::RawMessagesCommon::{diffArrow, diffLabels},
};

pub fn get_diff({
    before,
    after,
    id_label,
    show_full_message_for_diff: false,
}: {
    before: serde_json::Value,
    after: serde_json::Value,
    id_label: Option<&str>,
    show_full_message_for_diff: bool,
}) -> serde_json::Value {
    if let Ok(before_array) = serde_json::from_str::<Vec<serde_json::Value>>(before.to_string().as_str()) {
        if let Ok(after_array) = serde_json::from_str::<Vec<serde_json::Value>>(after.to_string().as_str()) {
            return diff_arrays(&before_array, &after_array, show_full_message_for_diff);
        }
    }

    if let Ok(before_object) = serde_json::from_str::<serde_json::Value>(before.to_string().as_str()) {
        if let Ok(after_object) = serde_json::from_str::<serde_json::Value>(after.to_string().as_str()) {
            return diff_objects(&before_object, &after_object, show_full_message_for_diff);
        }
    }

    if before == after {
        return serde_json::Value::Null;
    }

    if before.is_none() {
        return create_added_diff(after, id_label);
    }

    if after.is_none() {
        return create_deleted_diff(before, id_label);
    }

    return create_changed_diff(before, after);
}

fn diff_arrays(before: &[serde_json::Value], after: &[serde_json::Value], show_full_message_for_diff: bool) -> serde_json::Value {
    let mut diff = Vec::new();

    for i in 0..std::cmp::min(before.len(), after.len()) {
        let before_item = &before[i];
        let after_item = &after[i];

        if !is_valid_object_array(vec![before_item, after_item]) {
            continue;
        }

        let candidate_ids_to_compare_with = getCandidate_id_fields(&before_item as serde_json::Value);

        collect_candidate_values(before, candidate_ids_to_compare_with, "before");
        collect_candidate_values(after, candidate_ids_to_compare_with, "after");

        for (id_key, { before: candidate_id_before, after: candidate_id_after }) in candidate_ids_to_compare_with {
            if is_valid_id_field(candidate_id_before, candidate_id_after, before.len(), after.len()) {
                diff.push(process_diff_for_before_item(&before_item as serde_json::Value, id_key, i));
                break;
            }
        }

        if diff.is_empty() && show_full_message_for_diff {
            diff.push(create_added_diff(after_item, id_label));
        }
    }

    for i in (std::cmp::min(before.len(), after.len())..after.len()).rev() {
        let after_item = &after[i];

        if !is_valid_object_array(vec![before.get(i), after_item]) {
            continue;
        }

        let candidate_ids_to_compare_with = getCandidate_id_fields(&after_item as serde_json::Value);

        collect_candidate_values(before, candidate_ids_to_compare_with, "before");
        collect_candidate_values(after, candidate_ids_to_compare_with, "after");

        for (id_key, { before: candidate_id_before, after: candidate_id_after }) in candidate_ids_to_compare_with {
            if is_valid_id_field(candidate_id_before, candidate_id_after, before.len(), after.len()) {
                diff.push(process_diff_for_before_item(&after_item as serde_json::Value, id_key, i));
                break;
            }
        }

        if diff.is_empty() && show_full_message_for_diff {
            diff.push(create_deleted_diff(before.get(i), id_label));
        }
    }

    serde_json::Value::Array(diff)
}

fn diff_objects(before: &serde_json::Value, after: &serde_json::Value, show_full_message_for_diff: bool) -> serde_json::Value {
    let mut diff = serde_json::Value::Object(HashMap::new());

    for key in before.keys() {
        let before_value = before[key].clone();
        let after_value = after.get(key);

        if before_value != after_value {
            diff.insert(
                key.clone(),
                get_diff({
                    before: &before_value,
                    after: after_value.unwrap(),
                    id_label: None,
                    show_full_message_for_diff,
                }),
            );
        } else if show_full_message_for_diff {
            diff.insert(key.clone(), create_added_diff(&after_value, Some(key)));
        }
    }

    serde_json::Value::Object(diff)
}

fn is_not_object(value: &serde_json::Value) -> bool {
    return !value.is_array() && !value.is_null();
}

fn create_id_label_obj(value: &serde_json::Value, id_label: &str) -> serde_json::Value {
    serde_json::json!({
        diff_labels::ID.to_string(): serde_json::json!({
            id_label.to_string(): value[id_label.to_string()].clone(),
        }),
    })
}

fn create_labeled_diff(label_text: &str, value: &serde_json::Value, id_label: Option<&str>) -> serde_json::Value {
    if !id_label || is_not_object(value) {
        return serde_json::json!({ [label_text]: value.clone() });
    }

    let value_obj = value.as_ref().unwrap();
    let id_label_obj = create_id_label_obj(value_obj, id_label);

    serde_json::json!({
        label_text: serde_json::json!({
            ...id_label_obj,
            ...value_obj,
        }),
    })
}

fn create_added_diff(after: &serde_json::Value, id_label: Option<&str>) -> serde_json::Value {
    create_labeled_diff(diff_labels::ADDED.to_string(), after, id_label)
}

fn create_deleted_diff(before: &serde_json::Value, id_label: Option<&str>) -> serde_json::Value {
    create_labeled_diff(diff_labels::DELETED.to_string(), before, id_label)
}

fn create_changed_diff(before: &serde_json::Value, after: &serde_json::Value) -> serde_json::Value {
    let before_text = serde_json::json!(before.clone());
    let after_text = serde_json::json!(after.clone());

    serde_json::json!({
        diff_labels::CHANGED.to_string(): format!("{} {}", before_text, diff_arrow), // Assuming diff_arrow is a string
    })
}
```