```rust
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug)]
struct RosValue {
    // Define the fields of your RosValue struct here
}

#[derive(Debug)]
enum InteractionData {
    // Define the fields of your InteractionData struct here
}

type Props = {
    interaction_data: Option<InteractionData>,
    selected_object: RosValue,
    timezone: Option<&str>,
};

fn object_details(props: Props) -> impl Display {
    let json_tree_theme = use_json_tree_theme();
    let topic = props.interaction_data.as_ref().map(|d| d.topic);

    let original_object = serde_json::from_value(&serde_json::to_value(&props.selected_object).unwrap()).unwrap();

    if topic.is_none() {
        // show the original object directly if there is no interaction data
        return Box::new(ObjectDetails { interaction_data: None, selected_object, timezone });
    }

    Box::new(ObjectDetails {
        interaction_data,
        selected_object,
        timezone,
    })
}
```