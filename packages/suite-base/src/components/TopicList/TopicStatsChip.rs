```rust
use std::cell::{RefCell, RefMut};
use yew::prelude::*;
use yew_styles::mui as mui;

#[derive(PartialEq)]
pub enum TopicStat {
    Frequency,
    Count,
}

#[derive(Properties, PartialEq)]
pub struct TopicStatsChipProps {
    topic_name: String,
    selected: bool,
}

impl Properties for TopicStatsChipProps {}

struct Model {
    topic_name: RefCell<String>,
    selected: RefCell<bool>,
    stat_type: RefCell<Option<TopicStat>>,
}

struct State {
    model: Model,
}

#[function_component(TopicStatsChip)]
pub fn topic_stats_chip(props: &TopicStatsChipProps) -> Html {
    let props = props.clone();
    let state = use_state_with_cell(State::default);
    let model = &state.model;
    let selected = &mut state.selected;
    let stat_type = &mut state.stat_type;

    let classes = mui::use_styles!({
        root: {},
        selected: {},
        stat: {},
        divider: {},
    });

    let topic_name = props.topic_name.clone();
    let selected = props.selected;

    html! {
        <Paper variant="outlined" class={classes.root.clone()}>
            <div class={classes.stat.clone()} data-topic={topic_name} data-topic-stat="frequency">
                &ndash;
            </div>
            <Divider class={classes.divider.clone()} orientation="vertical" flex_item />
            <div class={classes.stat.clone()} data-topic={topic_name} data-topic-stat="count">
                &ndash;
            </div>
        </Paper>
    }
}
```