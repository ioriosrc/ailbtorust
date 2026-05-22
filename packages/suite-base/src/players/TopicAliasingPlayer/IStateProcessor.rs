```rust
use super::players::{PlayerState, SubscribePayload};

/**
 * IStateProcessor interface describes operations to transform PlayerState and Subscriptions.
 */
pub trait IStateProcessor {
  /**
   * Process a player state into a new player state.
   *
   * @param playerState the input player state
   * @param subs the latest subscriptions (for all topics including aliases)
   */
  fn process(player_state: &PlayerState, subs: &[SubscribePayload]) -> PlayerState;

  /**
   * Convert a set of subscriptions for all topics (including aliases) into subscriptions for only
   * the original topics.
   */
  fn alias_subscriptions(subs: &[SubscribePayload]) -> Vec<SubscribePayload>;
}
```