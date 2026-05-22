```rust
use chrono::{DateTime, Utc};
use lichtblick_rstime::{subtract as subtract_times, to_sec, Time};

type PlaybackControlsTooltipItem =
  | { type: "divider" }
  | { type: "item"; title: &str; value: &str };

pub fn playback_controls_tooltip_content(params: {
  stamp: DateTime<Utc>,
}) -> Option<Html> {
    let stamp = params.stamp;
    let time_format = use_app_time_format();
    let hovered_events = use_timeline_interaction_state(select_hovered_events);
    let startTime = use_message_pipeline(select_start_time);

    if !startTime.is_some() {
        return None;
    }

    let time_from_start = subtract_times(stamp, startTime.unwrap());

    let tooltip_items: Vec<PlaybackControlsTooltipItem> = Vec::new();

    if !hovered_events.is_empty() {
        for event in hovered_events.values().iter() {
            tooltip_items.push({
                type: "item",
                title: &event.start_time.to_string(),
                value: format_time(&event.end_time),
            });
            if !event.metadata.is_empty() {
                for (meta_key, meta_value) in event.metadata.iter() {
                    tooltip_items.push({
                        type: "item",
                        title: meta_key,
                        value: meta_value,
                    });
                }
            }
            tooltip_items.push({ type: "divider" });
        }
    }

    match time_format {
        "TOD" => tooltip_items.push({
            type: "item",
            title: &format_date(&stamp),
            value: &format_time(&stamp),
        }),
        "SEC" => tooltip_items.push({
            type: "item",
            title: &format_time(&stamp),
            value: &format_time(&stamp),
        }),
    }

    tooltip_items.push({
        type: "item",
        title: &to_sec(time_from_start).to_string(),
        value: &to_sec(time_from_start).to_string(),
    });

    let html = tooltip_items
        .iter()
        .map(|item| {
            if item.type == "divider" {
                Html::div(
                    attr!("class", "tooltip-divider"),
                    text!("\n\n"),
                )
            } else {
                Html::div(
                    attr!("key", &format!("{:?}", item)),
                    Html::text(&format!("<span class=\"item-key\">{}</span>\n<time>{}</time>", item.title, item.value)),
                )
            }
        })
        .collect();

    Some(html)
}
```