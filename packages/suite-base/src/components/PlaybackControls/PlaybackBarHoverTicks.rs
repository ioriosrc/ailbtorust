```rust
use crate::components::{Chart, HoverBar};
use crate::hooks::{
    useAppTimeFormat,
    useMessagePipeline,
    useHoverValue,
};

fn get_start_time(ctx: &crate::message_pipeline::Context) -> Option<chrono::Duration> {
    ctx.player_state.active_data.as_ref().map(|data| data.start_time)
}

fn get_end_time(ctx: &crate::message_pipeline::Context) -> Option<chrono::Duration> {
    ctx.player_state.active_data.as_ref().map(|data| data.end_time)
}

#[derive(Debug, Default)]
pub struct PlaybackBarHoverTicksProps {
    component_id: String,
}

pub fn playback_bar_hover_ticks(props: PlaybackBarHoverTicksProps) -> impl FnOnce(&crate::components::Stack) + '_ {
    let mut cache = std::collections::HashMap::new();
    move |stack| {
        stack.add_child(Chart::default());
        if !cache.contains_key(&props.component_id) {
            let start_time = use_message_pipeline(get_start_time);
            let end_time = use_message_pipeline(get_end_time);
            let hover_value = use_hover_value(|_| Some((props.component_id.clone(), 0.5))); // example hover value
            let format_time = use_app_time_format();

            cache.insert(props.component_id.clone(), start_time, end_time, hover_value);

            stack.add_child(HoverBar::new(
                props.component_id.clone(),
                cache[&props.component_id],
                cache.get(&props.component_id).unwrap().end_time,
                0.5, // example scale bounds
            ));
        }
    }
}
```