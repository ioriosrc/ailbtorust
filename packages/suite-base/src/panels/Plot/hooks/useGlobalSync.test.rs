```rust
use std::collections::HashMap;

// Define the PlotCoordinator struct and its methods mock implementation
struct PlotCoordinator {
    set_global_bounds: Box<dyn Fn(&mut HashMap<String, f64>)>,
    on: Box<dyn Fn(String, fn(&mut HashMap<String, f64>))>,
    off: Box<dyn Fn(String, fn(&mut HashMap<String, f64>))>,
}

impl PlotCoordinator {
    pub fn new() -> Self {
        PlotCoordinator {
            set_global_bounds: Box::new(|bounds| {}),
            on: Box::new(|event, handler| {}),
            off: Box::new(|event, handler| {}),
        }
    }

    pub fn set_global_bounds(&mut self, bounds: HashMap<String, f64>) {
        (self.set_global_bounds)(bounds);
    }

    pub fn on(&mut self, event: String, handler: fn(&mut HashMap<String, f64>)) {
        (self.on)(event, handler);
    }

    pub fn off(&mut self, event: String, handler: fn(&mut HashMap<String, f64>)) {
        (self.off)(event, handler);
    }
}

// Define the useGlobalSync function
fn use_global_sync(coordinator: Option<&PlotCoordinator>, set_can_reset: &mut dyn Fn(bool), options: &HashMap<String, bool>, subscriber_id: String) -> () {
    if let Some(coordinator) = coordinator {
        if *options.get("shouldSync").unwrap() && subscriber_id != coordinator.source_id {
            coordinator.set_global_bounds(global_bounds);
        }
    }
    set_can_reset(true); // Assuming setCanReset is a mock function that sets can reset to true
}

// Test cases for useGlobalSync function
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{PlotCoordinator, use_global_sync};

    fn render_use_global_sync() -> () {
        // Implementation of rendering the useGlobalSync hook using testing-library's renderHook
    }

    #[test]
    fn test_set_global_bounds_on_coordinator_if_should_sync_is_true_and_source_id_is_different() {
        let mut coordinator = PlotCoordinator::new();
        coordinator.source_id = "source1".to_string();

        render_use_global_sync(&coordinator, &mut |can_reset| can_reset(true), &HashMap::from([("shouldSync", true)]), "subscriber1");

        assert_eq!(coordinator.set_global_bounds.call_count(), 1);
    }

    #[test]
    fn test_not_set_global_bounds_on_coordinator_if_should_sync_is_false() {
        let mut coordinator = PlotCoordinator::new();

        render_use_global_sync(&coordinator, &mut |can_reset| can_reset(false), &HashMap::from([("shouldSync", false)]), "subscriber1");

        assert_eq!(coordinator.set_global_bounds.call_count(), 0);
    }

    #[test]
    fn test_not_set_global_bounds_on_coordinator_if_source_id_is_the_same() {
        let mut coordinator = PlotCoordinator::new();
        coordinator.source_id = "source1".to_string();

        render_use_global_sync(&coordinator, &mut |can_reset| can_reset(true), &HashMap::from([("shouldSync", true)]), "subscriber1");

        assert_eq!(coordinator.set_global_bounds.call_count(), 1);
    }

    #[test]
    fn test_add_and_remove_event_listeners_on_coordinator() {
        let mut coordinator = PlotCoordinator::new();

        render_use_global_sync(&coordinator, &mut |can_reset| can_reset(true), &HashMap::from([("shouldSync", true)]), "subscriber1");

        assert_eq!(coordinator.on.call_count(), 2);
    }

    #[test]
    fn test_call_set_global_bounds_on_timeseries_bounds_event() {
        let mut coordinator = PlotCoordinator::new();
        coordinator.source_id = "source1".to_string();

        render_use_global_sync(&coordinator, &mut |can_reset| can_reset(true), &HashMap::from([("shouldSync", true)]), "subscriber1");
        let new_bounds = HashMap::from([
            ("min", 10.0),
            ("max", 90.0),
        ]);

        let timeseries_bounds_handler = coordinator.on
            .as_ref()
            .unwrap()
            .calls
            .find(|(event, _)| event == "timeseriesBounds")
            .unwrap()[1];

        timeseries_bounds_handler(&mut coordinator.global_bounds);

        assert_eq!(coordinator.set_global_bounds.call_count(), 1);
    }

    #[test]
    fn test_not_call_set_global_bounds_when_coordinator_is_undefined() {
        let mut coordinator = PlotCoordinator::new();
        coordinator.source_id = "source1".to_string();

        render_use_global_sync(&None, &mut |can_reset| can_reset(true), &HashMap::from([("shouldSync", true)]), "subscriber1");

        assert_eq!(coordinator.set_global_bounds.call_count(), 0);
    }
}
```