```rust
use @lichtblick/message-path as path;
use @lichtblick/suite as suite;
use @lichtblick/suite-base::players::types::{SubscribePayload, SubscriptionPreloadType};

pub fn path_to_subscribe_payload(
    path: Immutable<path::MessagePath>,
    preload_type: SubscriptionPreloadType,
) -> Option<SubscribePayload> {
    let { message_path: parts, topic_name: topic } = path.into();

    let first_field: Option<path::MessagePathName> = parts
        .iter()
        .find(|part| part.type_() == "name")
        .map(|part| part.into());

    if first_field.is_none() || first_field.as_ref().unwrap().name().len() == 0 {
        return None;
    }

    // Always subscribe to the header so it is available for header stamp mode
    let fields = std::collections::HashSet::from(["header", &first_field.unwrap().name()]);

    for part in parts.iter() {
        if part.type_() != "filter" {
            break;
        }

        let { path: filter_path } = part.into();
        if filter_path.is_empty() {
            continue;
        }

        fields.insert(filter_path[0]);
    }

    Some(SubscribePayload {
        topic,
        preload_type,
        fields: fields.into_iter().collect(),
    })
}
```