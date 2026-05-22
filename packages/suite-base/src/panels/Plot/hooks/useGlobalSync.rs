```rust
use std::rc::Rc;

use parking_lot::Mutex;

use chrono::{DateTime, Duration};
use serde_json::Value;

use crate::{
    components::time_based_chart::types::Immutable,
    context::TimelineInteractionStateContext,
    panels::plot::PlotCoordinator,
};

fn select_global_bounds(store: &Rc<TimelineInteractionStateStore>) -> Option<Immutable<Bounds1D>> {
    store.global_bounds.as_ref().cloned()
}

fn select_set_global_bounds(store: &Rc<TimelineInteractionStateStore>) -> impl Fn(Immutable<Bounds1D>) + 'static {
    move |bounds| {
        let mut lock = store.write();
        if !lock.global_bounds.is_none() && lock.global_bounds.as_ref().unwrap().source_id == bounds.source_id {
            return;
        }
        lock.set_global_bounds(bounds);
    }
}

fn use_global_sync(
    coordinator: Option<&PlotCoordinator>,
    set_can_reset: Rc<dyn Fn(bool) + Send + Sync>,
    { should_sync }: { should_sync: bool },
    subscriber_id: String,
) -> () {
    let global_bounds = use_timeline_interaction_state(Rc::clone(&select_global_bounds));
    let set_global_bounds = use_timeline_interaction_state(select_set_global_bounds);

    useEffect(
        move || {
            if let Some(coordinator) = coordinator {
                if global_bounds.is_some() && global_bounds.as_ref().unwrap().source_id == subscriber_id
                    || !should_sync
                {
                    return;
                }
                coordinator.set_global_bounds(global_bounds);
            }
        },
        [coordinator, global_bounds.as_ref(), should_sync, subscriber_id],
    );

    useEffect(
        move || {
            if let Some(coordinator) = coordinator {
                let on_timeseries_bounds = move |new_bounds: Immutable<Bounds1D>| {
                    set_global_bounds(new_bounds);
                };

                let on_viewport_change = move |_event| {
                    set_can_reset(true);
                };

                coordinator.on("timeseriesBounds", on_timeseries_bounds);
                coordinator.on("viewportChange", on_viewport_change);

                return || {
                    coordinator.off("timeseriesBounds", on_timeseries_bounds);
                    coordinator.off("viewportChange", on_viewport_change);
                };
            }
        },
        [coordinator, set_can_reset.clone(), set_global_bounds],
    );
}
```