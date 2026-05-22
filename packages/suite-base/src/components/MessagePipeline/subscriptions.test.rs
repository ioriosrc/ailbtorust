```rust
fn merge_subscriptions(subs: Vec<InternalSubscribePayload>) -> Vec<InternalSubscribePayload> {
    let mut result = Vec::new();
    
    for sub in subs {
        if result.is_empty() || !result.iter().any(|x| x.topic == sub.topic && x.preload_type == sub.preload_type) {
            result.push(sub);
        } else if sub.fields.is_none() {
            // If the current subscription has no fields, keep it.
            result.push(sub);
        }
    }
    
    result
}
```