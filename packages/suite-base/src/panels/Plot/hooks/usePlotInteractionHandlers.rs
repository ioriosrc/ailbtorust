```rust
use react::prelude::*;
use react_use::useMountedState;

use @lichtblick/den/async::{debounce, debounce_async};
use @lichtblick/suite-base/components/MessagePipeline/use_message_pipeline_getter.rs as use_message_pipeline_getter;
use @lichtblick/suite-base/context/TimelineInteractionStateContext/use_timeline_interaction_state_context.rs as use_timeline_interaction_state_context;
use @lichtblick/suite-base/panels/Plot/types.rs as plot_types;
use @lichtblick/suite-base/util/layout.rs as layout;
use @lichtblick/suite-base/panels/Plot/utils/csv.rs as csv_utils;

fn select_set_global_bounds(store: &TimelineInteractionStateStore) -> Option<()> {
    store.set_global_bounds()
}

const DEFAULT_CSV_TITLE = "plot_data";

fn use_plot_interaction_handlers(props: plot_types::UsePlotInteractionHandlersProps) -> plot_types::UsePlotInteractionHandlers {
    let set_hover_value = use_set_hover_value();
    let clear_hover_value = use_clear_hover_value();
    let is_mounted = use_mounted_state();
    let mouse_present_ref = useRef(false);
    let { xAxis_val: x_axis_mode, [layout::PANEL_TITLE_CONFIG_KEY]: custom_title } = props.config;
    let set_global_bounds = use_timeline_interaction_state_context(props.timeline_interaction_state_store).select(select_set_global_bounds);
    let get_message_pipeline_state = use_message_pipeline_getter();
    let [focused_path, set_focused_path] = useState<Option<String>>();

    let build_tooltip = useMemo(() => {
        return debounce_async(async (args: plot_types::ElementAtPixelArgs) => {
            if (!is_mounted()) {
                return;
            }

            // Looking up a tooltip is an async operation so the mouse might leave the component while
            // that is happening and we need to avoid showing a tooltip.
            if (!args.elements || args.elements.is_empty() || !mouse_present_ref.current) {
                set_hover_value(None);
                return;
            }

            let tooltip_items: Vec<plot_types::TimeBasedChartTooltipData> = args.elements.iter().map(|element| {
                let value = element.data.value.unwrap_or(element.data.y);
                let tooltip_value = if value.is_number() && value >= 0.0 { from_sec(value) } else { value };

                plot_types::TimeBasedChartTooltipData {
                    config_index: element.config_index,
                    value: tooltip_value,
                }
            }).collect();

            set_hover_value(Some(plot_types::HoverValue {
                component_id: props.subscriber_id.to_string(),
                value: seconds,
                type: if x_axis_mode == "timestamp" { "PLAYBACK_SECONDS".to_string() } else { "OTHER".to_string() },
            }));
        });
    }, [props.renderer, is_mounted, set_hover_value, props.subscriber_id, x_axis_mode]);

    let onMouse_move = useCallback(
        (event: React.MouseEvent<HTMLElement>) => {
            mouse_present_ref.current = true;
            let bounding_rect = event.currentTarget.getBoundingClientRect();
            build_tooltip({
                clientX: event.clientX,
                clientY: event.clientY,
                canvasX: event.clientX - bounding_rect.left,
                canvasY: event.clientY - bounding_rect.top,
            });

            if (!props.coordinator) {
                return;
            }

            let rect = event.currentTarget.getBoundingClientRect();
            let mouseX = event.clientX - rect.left;
            let seconds = props.coordinator.get_x_value_at_pixel(mouseX);

            set_hover_value({
                component_id: props.subscriber_id.to_string(),
                value: seconds,
                type: x_axis_mode == "timestamp" ? "PLAYBACK_SECONDS".to_string() : "OTHER".to_string(),
            });
        },
        [build_tooltip, props.coordinator, set_hover_value, props.subscriber_id, x_axis_mode],
    );

    let onMouse_out = useCallback(() => {
        mouse_present_ref.current = false;
        set_hover_value(None);
        clear_hover_value(props.subscriber_id);
    }, [clear_hover_value, props.subscriber_id, set_hover_value]);

    let on_wheel = useCallback(
        (event: React.WheelEvent<HTMLElement>) => {
            if (!props.coordinator) {
                return;
            }

            let bounding_rect = event.currentTarget.getBoundingClientRect();
            props.coordinator.add_interaction_event({
                type: "wheel",
                cancelable: false,
                deltaY: event.deltaY,
                deltaX: event.deltaX,
                clientX: event.clientX,
                clientY: event.clientY,
                boundingClientRect: bounding_rect.toJSON(),
            });
        },
        [props.coordinator],
    );

    let on_reset_view = useCallback(() => {
        if (!props.coordinator) {
            return;
        }

        props.coordinator.reset_bounds();

        if (props.should_sync) {
            set_global_bounds(None);
        }
    }, [props.coordinator, set_global_bounds, props.should_sync]);

    let onClick = useCallback(
        (event: React.MouseEvent<HTMLElement>): void => {
            // If we started a drag we should not register a seek
            if (props.dragging_ref.current) {
                return;
            }

            // Only timestamp plots support click-to-seek
            if (x_axis_mode != "timestamp" || !props.coordinator) {
                return;
            }

            let {
                seek_playback,
                player_state: { active_data: { startTime: start } = {} },
            } = get_message_pipeline_state();

            if (!seek_playback || !start) {
                return;
            }

            let rect = event.currentTarget.getBoundingClientRect();
            let mouseX = event.clientX - rect.left;

            let seek_seconds = props.coordinator.get_x_value_at_pixel(mouseX);
            // Avoid normalizing a negative time if the clicked point had x < 0.
            if (seek_seconds >= 0) {
                seek_playback(add_times(start, from_sec(seek_seconds)));
            }
        },
        [props.coordinator, props.dragging_ref, get_message_pipeline_state, x_axis_mode],
    );

    let onClick_path = useCallback((index: usize) => {
        set_focused_path(Some(index.to_string()));
    }, []);

    let { key_down_handlers, key_up_handlers } = useMemo(() => {
        return {
            key_down_handlers: {
                v: () => {
                    props.coordinator.set_zoom_mode("y");
                },
                b: () => {
                    props.coordinator.set_zoom_mode("xy");
                },
            },
            key_up_handlers: {
                v: () => {
                    props.coordinator.set_zoom_mode("x");
                },
                b: () => {
                    props.coordinator.set_zoom_mode("x");
                },
            },
        };
    }, [props.coordinator]);

    let on_download_csv_click = useCallback(() => {
        void (async () => {
            try {
                let data = await props.coordinator.get_csv_data();
                if (!data || !is_mounted()) {
                    return;
                }

                csv_utils::download_csv(custom_title.unwrap_or(DEFAULT_CSV_TITLE), data, x_axis_mode);
            } catch (err: unknown) {
                console.error(err);
            }
        })();
    }, [props.coordinator, custom_title, is_mounted, x_axis_mode]);

    let get_panel_context_menu_items = useCallback(() => {
        let items: plot_types::PanelContextMenuItem[] = [
            {
                type: "item",
                label: "Download plot data as CSV",
                onclick: on_download_csv_click,
            },
        ];
        return items;
    }, [on_download_csv_click]);

    return {
        onMouse_move,
        onMouse_out,
        on_wheel,
        on_reset_view,
        onClick,
        onClick_path,
        focused_path,
        key_down_handlers,
        key_up_handlers,
        get_panel_context_menu_items,
    };
}
```

Note: The Rust code does not use TypeScript for its type annotations. However, the function signatures and variable types are similar to their TypeScript counterparts.