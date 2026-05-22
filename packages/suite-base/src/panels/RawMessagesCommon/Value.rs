```rust
use std::rc::Rc;

use materialize::{prelude::*};
use materialize::themes::*;

fn Value(props: PropsValue) -> Element {
    let (copied, set_copied) = useState(false);

    let value_color = useMemo(
        || get_value_color(&props.item_value, props.theme.mode),
        &props.item_value,
        &[&props.theme],
    );

    let copy_action = Rc::new(get_copy_action(
        copied.clone(),
        &props.item_value,
        move |value| {
            clipboard.copy(value)
                .then(|_| set_copied(true))
                .catch(|e: std::error::Error| println!("Failed to copy value: {}", e));
        },
    ));

    let open_plot_panel = Rc::new(move |path_suffix: &str| {
        props.on_topic_path_change(&format!("{}/{}", props.base_path, path_suffix));
    });

    let open_state_transitions_panel = Rc::new(move |path_suffix: &str| {
        props.open_sibling_state_transitions_panel(&format!("{}/{}", props.base_path, path_suffix));
    });

    let filter_action = Rc::new(get_filter_action(
        move || props.on_topic_path_change(&format!("{}{}", props.base_path, props.value_action.filter_path)),
    ));

    let available_actions: Vec<ValueActionItem> = if props.arr_label.len() > 0 {
        vec![copy_action.clone()]
    } else {
        vec![]
    };

    if props.value_action.is_some()
        && (props.value_action.primitive_type == PLOTABLE_ROS_TYPES
            || props.value_action.primitive_type == TRANSITIONABLE_ROS_TYPES)
    {
        let is_multi_slice_path = props.value_action.multi_slice_path == props.value_action.single_slice_path;

        available_actions.extend(vec![
            Rc::new(get_line_chart_action(
                &props.value_action.single_slice_path,
                open_plot_panel.clone(),
            )),
            if !is_multi_slice_path {
                Rc::new(get_scatter_plot_action(
                    &props.value_action.multi_slice_path,
                    open_plot_panel.clone(),
                ))
            } else {
                Vec::new()
            },
        ]);
    }

    if props.value_action.is_some() && props.value_action.primitive_type == TRANSITIONABLE_ROS_TYPES
        && is_multi_slice_path
    {
        available_actions.extend(vec![Rc::new(get_state_transitions_action(
            &props.value_action.single_slice_path,
            open_state_transitions_panel.clone(),
        ))]);
    }

    let placeholder_actions_for_spacing: Vec<ValueActionItem> = (0..MAX_ACTION_ITEMS)
        .map(|i| {
            ValueActionItem {
                key: format!("empty-{}", i),
                tooltip: "Tooltip",
                icon: Materialize::Icon::Error,
            }
        })
        .collect();

    let classes = use_theme().get_current_color("primary");

    let pointer_over = useState(false);

    html! {
        <Stack inline flex_wrap="wrap" direction="row" align_items="center" gap={0.25}>
            <span style={value_color.as_ref()}>
                {HighlightedValue::new(&props.item_label)}
            </span>
            {props.arr_label}
            {pointer_over && available_actions.iter().map(|action| {
                html! {
                    <Tooltip key={action.key} arrow title={action.tooltip} placement="top">
                        <MaterializeIconButton
                            size="small"
                            active_color=action.active_color.as_ref()
                            onClick=move |_| action.clone().perform()
                            color="inherit"
                            icon=action.icon.as_ref()
                        />
                    </Tooltip>
                }
            })}
            <span class={classes.placeholder_action_container}>
                {pointer_over && placeholder_actions_for_spacing.iter().map(|action| {
                    html! {
                        <Tooltip key={action.key} arrow title={action.tooltip} placement="top">
                            <MaterializeIconButton
                                size="small"
                                color="inherit"
                                icon=action.icon.as_ref()
                            />
                        </Tooltip>
                    }
                })}
            </span>
        </Stack>
    }
}

// In practice this seems to be an expensive component to render.
// Memoization provides a very noticeable performance boost.
export default Value;
```