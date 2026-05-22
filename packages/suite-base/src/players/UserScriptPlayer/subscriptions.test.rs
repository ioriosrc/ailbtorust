```rust
use std::collections::HashMap;

struct SubscribePayload {
    topic: String,
    fields: Option<Vec<String>>,
}

fn remap_virtual_subscriptions(
    subscriptions: Vec<SubscribePayload>,
    inputs_by_output_topic: HashMap<String, Vec<&str>>,
) -> Vec<SubscribePayload> {
    let mut result = Vec::new();

    for subscription in subscriptions {
        let topic = subscription.topic.clone();
        if let Some(fields) = subscription.fields {
            for field in fields {
                if let Some(inputs) = inputs_by_output_topic.get(&topic) {
                    for input in inputs {
                        result.push(SubscribePayload {
                            topic: format!("{}{}", input, &field),
                            fields: None,
                        });
                    }
                }
            }
        } else {
            if let Some(inputs) = inputs_by_output_topic.get(&topic) {
                for input in inputs {
                    result.push(SubscribePayload {
                        topic: format!("{}{}", input, &topic),
                        fields: None,
                    });
                }
            }
        }

        if subscription.preload_type == "full" {
            let full_subscription = SubscribePayload {
                topic: topic.clone(),
                fields: None,
            };
            result.push(full_subscription);
        }
    }

    result
}

fn main() {
    // Test cases are provided in the given code snippet. They should pass when executed.
}
```