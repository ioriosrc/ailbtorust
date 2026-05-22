```rust
use mui::{
    components::{Checkbox, FormControlLabel},
    theme::createTheme,
};
use react::prelude::*;
use std::rc::Rc;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

type PropsRawMessagesVirtual = {
    palette: { mode: String };
    classes: { [key: &str]: String };
    config: { topicPath: String; diffMethod: String; diffTopicPath: String; diffEnabled: bool; showFullMessageForDiff: bool; fontSize: String };
};

type NodeState = bool;
type TreeNode = {
    value: AnyValue;
    keyPath: Vec<String>;
};

fn useSharedRawMessagesLogic(props: PropsRawMessagesVirtual) -> Rc<PropsRawMessagesVirtual> {
    // Implement the logic for shared raw messages
    Rc::new(props)
}

fn Main() -> JSXElement {
    let theme = createTheme({
        palette: { mode: "light" },
    });

    let state = useSharedRawMessagesLogic({
        topicPath: String::from("example"),
        diffMethod: String::from("custom_method"),
        diffTopicPath: String::from("example_diff"),
        diffEnabled: true,
        showFullMessageForDiff: false,
        fontSize: String::from("16px"),
    });

    let valueRenderer = useValueRenderer({
        datatypes: Some(vec!["string", "number"]),
        hoverObserverClassName: "hoverObserver",
        onTopicPathChange: |_| {},
        openSiblingPanel: |_| {},
    });

    // Implement the logic for rendering diff label
    fn renderDiffLabel(valueString: String, value: AnyValue) -> JSXElement {
        // Implement the logic for rendering diff label
        <div>{valueString}</div>
    }

    // VirtualizedTree-specific logic
    let handleToggleExpand = move |keyPath: &str| {
        // Implement the logic for handling expand toggle
    };

    let baseItemRef = Rc::new(None);

    let expandedNodesSet = useMemo(move || {
        if state.config.expansion == "all" {
            if (baseItemRef.is_none()) {
                return std::collections::HashSet::new();
            }

            let data = baseItemRef.as_ref().unwrap().queried_data
                .iter()
                .map(|item| item.value.clone())
                .collect::<Vec<AnyValue>>();

            let all_nodes = std::collections::HashSet::new();

            fn generate_paths(obj: &AnyValue, prefix: &str) {
                if obj.is_none() || !obj.is_object() {
                    return;
                }
                let entries = if obj.is_array() {
                    obj.as_array().iter().map(|item| (item.to_string(), item.clone()))
                } else {
                    obj.into_iter()
                        .collect::<Vec<(String, AnyValue)>>()
                };

                for (key, value) in entries {
                    let node_path = prefix.join(&key);
                    all_nodes.insert(node_path);

                    if !value.is_none() && value.is_object() && !value.is_array() {
                        generate_paths(value.as_ref(), &node_path);
                    }
                }
            }

            generate_paths(data);

            return all_nodes;
        }

        if (state.config.expansion == "none") {
            return std::collections::HashSet::new();
        }

        let expanded = std::collections::HashSet::new();
        for (&key, state) in &state.config.expansion {
            if state == NodeState::Expanded {
                expanded.insert(key.to_string());
            }
        }
        return expanded;
    }, [state.config.expansion]);

    let hide_wrapping_array_ref = Rc::new(false);

    let memoized_render_value = useMemo(move || {
        let value_string = |value: &AnyValue| -> String {
            // Implement the logic for rendering value string
            value.to_string()
        };

        let should_display_single_val = move || {
            let data = state.base_item.as_ref().unwrap().queried_data
                .iter()
                .map(|item| item.value.clone())
                .collect::<Vec<AnyValue>>();

            hide_wrapping_array_ref.is_none() && data.len() == 1 && data[0].is_object()
        };

        let single_val = move || {
            if state.base_item.as_ref().unwrap().queried_data.is_empty() {
                return None;
            }

            let value = &state.base_item.as_ref().unwrap().queried_data[0];
            if value.is_object() {
                return Some(value.clone());
            }
            Some(get_single_value(&data, &state.base_item.unwrap().queried_data))
        };

        move |node: &TreeNode, data: &AnyValue| -> JSXElement {
            let value_string = memoized_render_value(data);
            let diff_enabled = state.config.diff_enabled;
            let diff_message = if diff_enabled && state.config.diff_method == "custom_method" && state.base_item.is_none() {
                Some(state.config.diff_topic_path.clone())
            } else {
                None
            };

            let render_content = move || {
                if should_display_single_val() {
                    <div>
                        <MaybeCollapsedValue item_label={value_string} />
                    </div>
                }

                if diff_enabled && value.is_object() && !diff_message.is_none() {
                    <EmptyState>{format!("Waiting to diff next messages from {}", diff_message.unwrap())}</EmptyState>
                }

                return (
                    <>
                        {diff_enabled && (
                            <FormControlLabel
                                disableTypography
                                checked={state.config.show_full_message_for_diff}
                                control={
                                    <Checkbox
                                        size="small"
                                        defaultChecked
                                        onChange={() => {
                                            state.save_config({ show_full_message_for_diff: !state.config.show_full_message_for_diff });
                                        }}
                                    />
                                }
                                label="Show full message"
                            />
                        )}
                        <VirtualizedTree
                            data={diff_enabled ? value : data}
                            expanded_nodes={expanded_nodes_set}
                            on_toggle_expand={handle_toggle_expand}
                            fontSize={state.config.font_size.clone()}
                            render_value=|node: &TreeNode| memoized_render_value(node, data)
                        />
                    </>
                );
            };

            return (
                <Stack
                    class_name={classes.topic}
                    flex="auto"
                    overflowX="hidden"
                    paddingLeft={0.75}
                    data-testid="panel-scroll-container"
                >
                    <Metadata
                        data={data}
                        diff_data={value.is_object() ? value : None}
                        diff={value.is_object() && !diff_message.is_none() ? get_diff(&data, &state.base_item.unwrap().queried_data) : {}}
                        message={state.base_item.as_ref().unwrap().message_event.clone()}
                        datatype={Some(state.config.topic.schema_name.clone())}
                        diff_message={state.base_item.as_ref().unwrap().diff_message.clone()}
                    />
                    {render_content()}
                </Stack>
            );
        }
    }, [state.base_item, state.config.diff_enabled, state.config.diff_method, state.config.show_full_message_for_diff, state.config.font_size]);

    // Setup settings in panel settings tree
    use_panel_settings({
        fontSize: state.config.font_size.clone(),
        save_config: move |config| {
            state.save_config(config);
        },
    });

    <Stack flex="auto" overflow="hidden" position="relative">
        <Toolbar
            can_expand_all={state.config.can_expand_all}
            diff_enabled={state.config.diff_enabled}
            diff_method={state.config.diff_method}
            diff_topic_path={state.config.diff_topic_path}
            on_diff_topic_path_change=|path: String| {
                state.on_diff_topic_path_change(path);
            }
            on_toggle_diff=|enabled: bool| {
                state.on_toggle_diff(enabled);
            }
            on_toggle_expand_all=|enabled: bool| {
                state.on_toggle_expand_all(enabled);
            }
            on_topic_path_change=|path: String| {
                state.on_topic_path_change(path);
            }
            save_config=|config: PropsRawMessagesVirtual| {
                state.save_config(config);
            }
            topic_path={state.config.topic_path}
        />
        {memoized_render_value(state.base_item.as_ref().unwrap(), state.base_item.as_ref().unwrap().queried_data)}
    </Stack>
}

fn useValueRenderer(props: PropsValueRenderer) -> Rc<PropsValueRenderer> {
    // Implement the logic for value renderer
    Rc::new(props)
}
```