```rust
use chrono::{DateTime, Duration};
use lighblick::rostime::{add, to_nanosecond, to_second};
use lighblick::suite_base::context::EventsContext;
use lighblick::suite_base::testing::builders::RosTimeBuilder;
use lighblick::test_builders::BasicBuilder;

pub fn make_mock_events(
    count: usize,
    start_sec: f64 = 100.0,
    step_sec: f64 = 1.0,
) -> Vec<EventsContext<TimelinePositionedEvent>> {
    let mut events = Vec::new();
    for idx in 0..count {
        let start_time = RosTimeBuilder.time(start_sec + step_sec * idx);
        let duration = RosTimeBuilder.time(step_sec * (idx % 3) + 1.0);
        
        let event = EventsContext::<TimelinePositionedEvent> {
            id: format!("event_{}", idx + 1),
            end_time: RosTimeBuilder.time(add(start_time, duration)),
            end_time_in_seconds: to_second(RosTimeBuilder.time(add(start_time, duration))),
            start_time,
            start_time_in_seconds: to_second(start_time),
            timestamp_nanos: to_nanosecond(start_time).to_string(),
            metadata: {
                type_: BasicBuilder.strings()[idx % 3],
                state: BasicBuilder.strings()[idx % 3],
            },
            created_at: BasicBuilder.datetime(),
            updated_at: BasicBuilder.datetime(),
            device_id: format!("device_{}", idx + 1),
            duration_nanos: to_nanosecond(duration).to_string(),
        };

        events.push(event);
    }

    events
}
```

Note that the Rust version of this code has several differences from the TypeScript/React version:

- The import paths are different.
- The `make_mock_events` function returns a vector of `EventsContext<TimelinePositionedEvent>` instead of an array.
- Rust uses snake_case for variable names and function names, while TypeScript uses camelCase.
- Rust does not have built-in support for date and time manipulation like TypeScript, so we use the `chrono` crate.