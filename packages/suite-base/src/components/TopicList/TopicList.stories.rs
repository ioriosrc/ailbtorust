```rust
use crate::components::topic_list::{TopicList, TopicStats};
use crate::players::{PLAYER_CAPABILITIES, Topic};
use crate::rosmsg_msgs_common::*;
use crate::suite_base::message_pipeline::MockMessagePipelineProvider;
use crate::suite_base::types::TopicStats;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn log(message: &str);
}

fn main() -> Result<(), JsValue> {
    let topics = vec![
        Topic::new("/topic_1", "std_msgs/String"),
        Topic::new("/topic_2", "std_msgs/String"),
    ];

    let topic_stats = map!["/topic_1" => TopicStats {
        num_messages: 1234,
        first_message_time: { sec: 1, nsec: 0 },
        last_message_time: { sec: 2, nsec: 0 },
    }, "/topic_2" => TopicStats {
        num_messages: 3456,
        first_message_time: { sec: 1, nsec: 0 },
        last_message_time: { sec: 2, nsec: 0 },
    }];

    let capabilities = vec![PLAYER_CAPABILITIES::playback_control];

    log("Rendering TopicList...");

    JsValue::from_serde(&TopicList {
        capabilities,
        topics,
        datatypes: map![ros2humble::std_msgs_std_msgs_string::TYPE_NAME => type_name!(ros2humble::std_msgs_std_msgs_string)],
        topic_stats,
    })?;

    Ok(())
}
```