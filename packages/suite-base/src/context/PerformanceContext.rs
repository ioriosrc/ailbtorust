```rust
use std::time::{Instant, Duration};

type PerformanceMetricID = u32;

#[derive(Debug)]
struct PerformanceMetric {
    id: PerformanceMetricID,
    name: String,
    unit: String,
}

pub trait IPerformanceRegistry {
    fn register_metric(&mut self, metric: impl Into<PerformanceMetric>) -> PerformanceMetricID;
    fn unregister_metric(&mut self, metric_id: PerformanceMetricID);
    fn add_measurement(&mut self, metric_id: PerformanceMetricID, timestamp: Instant, value: f64);
    fn scope_timer(&self, metric_id: PerformanceMetricID) -> impl Drop + 'static {
        struct Timer<'a>(&'a mut dyn IPerformanceRegistry);
        impl Drop for Timer<'a> {
            fn drop(&mut self) {
                let duration = Instant::now() - self.0.add_measurement(self.0.register_metric(PerformanceMetric {
                    id: 1,
                    name: "Execution time".to_string(),
                    unit: "ms per frame".to_string(),
                }));
                self.0.add_measurement(self.0.register_metric(PerformanceMetric {
                    id: 2,
                    name: "Execution time".to_string(),
                    unit: "ms per frame".to_string(),
                }), Instant::now(), duration.as_secs_f64());
            }
        }

        Timer(self)
    }
}

struct PerformanceContext;

impl IPerformanceRegistry for PerformanceContext {
    fn register_metric(&mut self, metric: impl Into<PerformanceMetric>) -> PerformanceMetricID {
        let id = 1; // Implement registration logic
        println!("Registered metric {}", metric.into().name);
        id
    }

    fn unregister_metric(&mut self, _metric_id: PerformanceMetricID) {
        println!("Unregistered metric");
    }

    fn add_measurement(&mut self, metric_id: PerformanceMetricID, timestamp: Instant, value: f64) {
        println!("Added measurement for metric {}: {}", metric_id, value);
    }

    fn scope_timer(&self, _metric_id: PerformanceMetricID) -> impl Drop + 'static {
        struct Timer<'a>(&'a mut dyn IPerformanceRegistry);
        impl Drop for Timer<'a> {
            fn drop(&mut self) {
                let duration = Instant::now() - self.0.add_measurement(self.0.register_metric(PerformanceMetric {
                    id: 1,
                    name: "Execution time".to_string(),
                    unit: "ms per frame".to_string(),
                }));
                self.0.add_measurement(self.0.register_metric(PerformanceMetric {
                    id: 2,
                    name: "Execution time".to_string(),
                    unit: "ms per frame".to_string(),
                }), Instant::now(), duration.as_secs_f64());
            }
        }

        Timer(self)
    }
}

pub fn use_performance() -> impl IPerformanceRegistry {
    PerformanceContext
}
```