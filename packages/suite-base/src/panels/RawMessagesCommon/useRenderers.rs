```rust
use leptos::*;
use once_cell::sync::Lazy;

#[allow(clippy::needless_lifetimes)]
pub fn use_value_renderer(
    datatypes: &[Datatype],
    hover_observer_class_name: &str,
    on_topic_path_change: Callback<ItemPath, ()>,
    open_sibling_panel: Callback<Vec<String>, ()>,
) -> ValueRendererFn {
    let enum_mapping = Lazy::new(|| {
        let mut mapping = std::collections::HashMap::new();
        for datatype in datatypes {
            for field in datatype.fields.iter().flatten() {
                mapping.entry(datatype.datatype.clone()).or_insert_with(Vec::default).push((field.to_string(), None));
            }
        }
        mapping
    });

    ValueRendererFn(move |structure_item, data, queried_data, label, item_value, key_path| {
        let last_key_path = *key_path.last().unwrap();
        let mut value_action: Option<ValueAction> = None;
        if is_hovering() {
            value_action = get_value_action_for_value(
                &data[last_key_path],
                structure_item,
                key_path[..last_key_path].iter().rev(),
            );
        }

        let constant_name: Option<&str> = get_constant_name_by_key_path(&key_path, &queried_data);
        if let Some(structure_item) = structure_item {
            if let Some(child_structure_item) = get_structure_item_for_path(
                structure_item,
                key_path[..last_key_path].iter().rev(),
            ) {
                if let Some(key_path_index) = key_path.iter().position(|key| key.is_string()) {
                    let field = key_path[key_path_index];
                    if let Some(datatype) = child_structure_item.datatype.clone() {
                        constant_name = enum_mapping[&datatype].get(field).and_then(|value| value.as_ref());
                    }
                }
            }
        }
        let basePath = queried_data[last_key_path].path.clone().unwrap_or_default();
        let { arr_label, item_label } = get_value_labels({
            constant_name: constant_name,
            label,
            item_value,
            key_path: Vec::new(),
        });

        if let Some(item_value) = &item_value {
            return view! {
                <ObjectSummary value={item_value} />
            };
        }

        view! {
            <Value
                arr_label={arr_label}
                basePath={basePath}
                item_label={item_label}
                item_value={item_value}
                value_action={value_action}
                on_topic_path_change={on_topic_path_change.clone()}
                open_sibling_panel={open_sibling_panel.clone()}
            />
        }
    })
}

pub fn use_render_diff_label(
    on_topic_path_change: Callback<ItemPath, ()>,
    open_sibling_panel: Callback<Vec<String>, ()>,
) -> RenderDiffLabelFn {
    let render_diff_label = Lazy::new(|| {
        move |label: &str, item_value: &Value| {
            let { arr_label, item_label } = get_value_labels({
                constant_name: None,
                label: label.to_string(),
                item_value: item_value.clone().to_string(),
                key_path: Vec::new(),
            });

            view! {
                <Value
                    arr_label={arr_label}
                    basePath=""
                    item_label={item_label}
                    item_value={item_value}
                    value_action=None
                    on_topic_path_change={on_topic_path_change.clone()}
                    open_sibling_panel={open_sibling_panel.clone()}
                />
            }
        }
    });

    render_diff_label
}
```