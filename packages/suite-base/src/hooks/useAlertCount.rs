```rust
use crate::services::{AlertsContextStore, MessagePipelineContext};
use crate::components::MessagePipeline;

fn use_alert_count() -> &'static (Vec<PlayerAlert>, AlertsContextStore<'static>, usize) {
    let (player_alerts, player_state) = MessagePipeline.use_context::<MessagePipelineContext>();
    let session_alerts = AlertsContextStore::default().alerts();

    let mut alert_count = 0;
    for alert in &session_alerts {
        alert_count += 1;
    }
    for alert in player_alerts {
        alert_count += 1;
    }

    (&player_alerts, &session_alerts, alert_count)
}
```