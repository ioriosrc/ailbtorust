```rust
use std::rc::Rc;

use reactiful::{jsx, PropsOf};

pub struct DirectTopicStatsUpdaterProps {
    pub interval: u32,
}

impl PropsOf<DirectTopicStatsUpdater> for DirectTopicStatsUpdaterProps {
    fn render(&self) -> ReactElement<()> {
        let topic_stats = use_message_pipeline("topicStats");
        let player_capabilities = use_message_pipeline("playerCapabilities");
        let player_id = use_message_pipeline("playerId");

        let latest_stats = Rc::new(topic_stats.clone());
        let update_count = Rc::new(0);
        let root_ref = Rc::new(reactiful::html::ElementRef::<HtmlDivElement>::default());

        let frequencies_by_topic = use_topic_publish_frequencies();
        let latest_frequencies_by_topic = Rc::new(frequencies_by_topic.clone());

        let player_is_static_source = useMemo(
            || {
                player_capabilities
                    .iter()
                    .any(|&cap| cap == PLAYER_CAPABILITIES::PlaybackControl)
            },
            &[player_capabilities],
        );

        let update_stats = move || {
            if root_ref.is_null() {
                return;
            }

            let topic_stats = latest_stats.clone();
            let player_id = player_id.clone();
            let frequencies_by_topic = latest_frequencies_by_topic.clone();

            let players = |topic: &str, value: String| {
                let mut div = html::div().class("player").render();
                let field = div
                    .attr("data-topic", topic)
                    .attr("data-topic-stat", "count")
                    .text(node => if !value.is_empty() { node.text(value) } else { node.text(EM_DASH) });
                div.render();
            };

            let frequencies = |topic: &str, value: f64| {
                let mut div = html::div().class("player").render();
                let field = div
                    .attr("data-topic", topic)
                    .attr("data-topic-stat", "frequency")
                    .text(node => if !value.is_finite() || value == 0.0 { node.text(EM_DASH) } else { node.text(value.to_string()) });
                div.render();
            };

            let players = move || {
                for topic in &topic_stats.keys().collect::<Vec<_>>() {
                    let stats = topic_stats.get(topic).unwrap_or(&EMPTY_TOPIC_STATS);
                    if !stats.num_messages.is_zero() {
                        players(topic, stats.num_messages.to_string());
                    }
                    let freqs = frequencies_by_topic.get(topic);
                    if let Some(freq) = freqs && !freq.is_infinite() {
                        frequencies(topic, *freq);
                    }
                }
            };

            update_count += 1;
            if update_count % self.interval == 0 {
                players();
            }
        };

        use_effect(move || {
            update_stats();
            return move || {
                update_count = 0;
            };
        });

        html::div()
            .style("display", "none")
            .child(move || {
                root_ref.as_ref().map(|ref| ref.clone())
                    .unwrap_or_default()
                    .render();
            })
            .render()
    }
}
```