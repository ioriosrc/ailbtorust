```rust
use log::debug;

pub struct AnalyticsMetricsCollector {
    metadata: std::collections::HashMap<String, String | f64 | bool>,
    analytics: IAnalytics,
}

impl AnalyticsMetricsCollector {
    pub fn new(analytics: IAnalytics) -> Self {
        debug!("New AnalyticsMetricsCollector");
        Self { metadata: std::collections::HashMap::new(), analytics }
    }

    pub fn set_property(&mut self, key: String, value: String | f64 | bool) {
        self.metadata.insert(key, value);
    }

    pub fn log_event(&self, event: AppEvent, data: Option<std::collections::HashMap<String, String | f64 | bool>>) {
        let mut combined_data = std::collections::HashMap::new();
        combined_data.extend(self.metadata.clone());
        if let Some(d) = data {
            combined_data.extend(d);
        }
        self.analytics.log_event(event, combined_data);
    }

    pub fn player_constructed(&self) {
        self.log_event(AppEvent::PLAYER_CONSTRUCTED);
    }
}
```