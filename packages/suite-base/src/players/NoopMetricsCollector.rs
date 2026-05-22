```rust
use std::fmt;

pub struct NoopMetricsCollector {}

impl PlayerMetricsCollectorInterface for NoopMetricsCollector {
    fn set_property(&self, _key: &str, _value: impl fmt::Display) {
        // no-op
    }

    fn player_constructed(&self) {
        // no-op
    }
}
```