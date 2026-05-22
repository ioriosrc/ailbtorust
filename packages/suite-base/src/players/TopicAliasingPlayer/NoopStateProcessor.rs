```rust
use crate::suite_base::players::{PlayerState, SubscribePayload};

pub struct NoopStateProcessor;

impl IStateProcessor for NoopStateProcessor {
    fn process(&self, player_state: PlayerState) -> PlayerState {
        player_state
    }

    fn alias_subscriptions(&self, subs: SubscribePayload[]) -> SubscribePayload[] {
        subs
    }
}
```