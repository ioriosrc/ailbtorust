```rust
use leptos::{prelude::*, view};
use serde_json::Value;

#[component]
pub fn LogsSection(logs: &[UserScriptLog]) -> impl IntoView {
    let auto_scroll = use_state(false);
    let json_tree_theme = use_json_tree_theme();
    let value_color_map: HashMap<&str, &str> = HashMap::from([
        ("string", &json_tree_theme.base0B),
        ("number", &json_tree_theme.base09),
        ("boolean", &json_tree_theme.base09),
        ("object", &json_tree_theme.base08), // null
        ("undefined", &json_tree_theme.base08),
    ]);

    let list_ref = use_node("logs_section_list");

    view! {
        <Stack gap={0.5} padding={2}>
            {if logs.is_empty() {
                <Typography variant="body2" color="text.secondary">
                    No logs to display.
                </Typography>
                <Typography variant="body2" color="text.secondary">
                    Invoke <code>log(someValue)</code> in your Lichtblick Suite node code to see data printed
                    here.
                </Typography>
            } else {
                <List dense disablePadding ref={list_ref}>
                    {logs.iter().enumerate().map(|(idx, log)| {
                        let source = log.source.clone();
                        let value = log.value.clone();

                        let render_tree_obj = value.is_object() && !value.is_null();
                        if render_tree_obj {
                            view! {
                                <Tree hide_root data={&value} invert_theme={false} theme={json_tree_theme} />
                            }
                        } else {
                            view! {
                                <ListItemText
                                    primary={
                                        if value.is_none() || value.is_falsy() {
                                            String::from(value)
                                        } else {
                                            value.as_ref().unwrap_or_default()
                                        }
                                    }
                                    slot_props={{
                                        primary: {
                                            color: value_color_map.get(&value.to_string()).unwrap_or(&"text.primary"),
                                        },
                                    }}
                                />
                            }
                        }
                    })}
                </List>
            }
        </Stack>
    }
}
```