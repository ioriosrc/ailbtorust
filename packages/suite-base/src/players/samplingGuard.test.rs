```rust
use std::collections::HashMap;

fn apply_sampling_guard_to_subscription(subscription: SubscribePayload) -> SubscribePayload {
    if !subscription.samplingAuthorized {
        return SubscribePayload {
            topic,
            samplingAuthorized: None,
            samplingRequest: None,
        };
    }
    subscription
}

fn apply_sampling_guard_to_subscriptions(input: Vec<SubscribePayload>) -> Vec<SubscribePayload> {
    input.into_iter()
        .filter(|sub| sub.samplingAuthorized)
        .collect()
}
```