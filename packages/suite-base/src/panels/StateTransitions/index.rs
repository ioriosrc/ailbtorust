```rust
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use std::rc::Rc;

use crate::{
    chart::{Chart, ScatterPlot, TimeBasedChart},
    components::{
        panel::{Panel, PanelToolbar},
        stack::Stack,
    },
    decode_message_range::DecodedMessageRange,
    hooks::{
        message_pipeline_getter::use_message_pipeline_getter,
        messages_by_path::use_messages_by_path,
        state_transitions::{
            EMPTY_ITEMS_BY_PATH, STATE_TRANSITION_PLUGINS, use_state_transitions_data,
            use_state_transitions_time,
        },
        time_based_chart::{TimeBasedChartConfig, XAxisLabel, YAxis},
    },
    message_path::parse_message_path,
    math::add_times,
    messages::RustMessage,
};

use super::{StateTransitionPanelProps, StateTransitionConfig};

fn StateTransitions(props: StateTransitionPanelProps) -> Rc<dyn Panel> {
    let { config, save_config } = props;
    let classes = use_state_transitions_styles();

    let focused_path = use_focused_path(config);

    use_message_path_drop_config(save_config);

    let { startTime, currentTime_since_start, endTime_since_start } =
        use_state_transitions_time();

    let topics = use_topics_and_path_strings(config);

    let decoded_messages = use_decoded_message_range(topics, &topics);

    // When range data is active, skip useMessagesByPath subscriptions entirely
    // to avoid wasteful current-frame processing and decoding.
    let has_range_data = use_has_range_data(&decoded_messages, topics);

    let items_by_path = if has_range_data {
        Rc::new(EMPTY_ITEMS_BY_PATH)
    } else {
        Rc::new(use_messages_by_path(&topics))
    };

    let height = use_height_and_height_per_topic(&topics);
    let new_items_by_path = has_range_data { EMPTY_ITEMS_BY_PATH: items_by_path.clone() };

    let show_points = config.show_points == true;

    let (path_state, data, minY) = use_state_transitions_data(
        &topics,
        startTime,
        &new_items_by_path,
        decoded_messages,
        show_points,
    );

    let { y_scale, x_scale, databounds, width } = use_chart_scales_and_bounds(
        minY,
        currentTime_since_start,
        endTime_since_start,
        config,
    );

    let message_pipeline = use_message_pipeline_getter();

    let onClick = Rc::new(move |x: f64| {
        let {
            seek_playback,
            player_state: { active_data: { startTime: start } = {} },
        } = message_pipeline();
        if !seek_playback || x.is_nan() || start.is_none() {
            return;
        }
        let seek_time = add_times(start.unwrap(), from_sec(x));
        seek_playback(seek_time);
    });

    use_panel_settings(config, save_config, path_state, focused_path);

    Rc::new(Stack {
        flex_grow: 1,
        overflow: "hidden",
        style: classes.chart_wrapper,
        children: vec![
            PanelToolbar {},
            Stack {
                full_width: true,
                full_height: true,
                flex: "auto",
                overflow_x: "hidden",
                overflow_y: "auto",
                children: vec![Box::new(TimeBasedChart::new(
                    TimeBasedChartConfig {
                        zoom: true,
                        is_synced: config.is_synced,
                        show_x_axis_labels: true,
                        width: width.unwrap_or(0.0),
                        height: height.0,
                        data: data.clone(),
                        data_bounds: databounds.clone(),
                        reset_button_padding_bottom: 2.0,
                        type_: "scatter",
                        x_axes: Rc::new(XAxisLabel {
                            topics,
                            height_per_topic: height.1,
                        }),
                        xAxis_is_playback_time: true,
                        y_axes: Rc::new(YAxis {
                            min_value: minY,
                        }),
                        plugins: Rc::new(STATE_TRANSITION_PLUGINS),
                        interaction_mode: "lastX",
                        onClick,
                    },
                ))],
            },
            Box::new(PathLegend::new(
                paths.clone(),
                height_per_topic,
                set_focused_path,
                save_config,
            )),
        ],
    })
}

fn use_focused_path(config: &StateTransitionConfig) -> Rc<dyn FnMut(String)> {
    Rc::new(move |path| config.focused_path = Some(path))
}

fn use_topics_and_path_strings(config: &StateTransitionConfig) -> (Vec<String>, Vec<String>) {
    let mut topics = vec![];
    let mut path_strings = vec![];

    for transition in &config.paths {
        if let Ok(parsed) = parse_message_path(&transition.value) {
            topics.push(parsed.topic_name.clone());
            path_strings.push(transition.value.clone());
        }
    }

    (topics, path_strings)
}

fn use_has_range_data(decoded_messages: &[DecodedMessageRange], topics: &[String]) -> bool {
    decoded_messages
        .iter()
        .any(|block| block.iter().any(|message| topics.contains(&message.topic_name)))
}

fn use_height_and_height_per_topic(topics: &[String]) -> (u32, f64) {
    let only_topics_height = topics.len() * 64;
    let xAxis_height = 30;
    (Math::max(80, only_topics_height + xAxis_height), topics.len() as f64)
}

fn use_chart_scales_and_bounds(
    minY: f64,
    currentTime_since_start: f64,
    endTime_since_start: f64,
    config: &StateTransitionConfig,
) -> (f64, f64, DecodedMessageRange, u32) {
    let y_scale = config.y_scale;
    let x_scale = config.x_scale;
    let databounds = config.databounds;
    let width = config.width.unwrap_or(0.0);

    (y_scale, x_scale, databounds, width)
}

fn use_state_transitions_data(
    topics: &[String],
    startTime: f64,
    items_by_path: &Rc<dyn MessagesByPath>,
    decoded_messages: &[DecodedMessageRange],
    show_points: bool,
) -> (StateTransitionConfig::PathState, Vec<RustMessage>, f64) {
    let path_state = config.path_state;
    let data = config.data;
    let minY = config.min_y;

    (path_state, data, minY)
}

fn use_message_pipeline_getter() -> Rc<dyn MessagePipelineGetter> {
    Rc::new(MessagePipelineGetter {})
}
```