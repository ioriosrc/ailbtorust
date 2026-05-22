```rust
use std::f64;

type UseStateTransitionsTime = {
    startTime: Option<time::Instant>,
    currentTimeSinceStart: f64,
    endTimeSinceStart: f64,
};

fn useStateTransitionsTime() -> UseStateTransitionsTime {
    let message_pipeline_state = use_message_pipeline_getter();

    let player_state = message_pipeline_state.player_state.as_ref().unwrap();
    let active_data = player_state.active_data;

    let currentTime = active_data.current_time;
    let startTime = active_data.start_time;

    let currentTime_since_start = if let Some(current) = currentTime {
        if let Some(start) = startTime {
            (current - start).as_secs_f64()
        } else {
            None
        }
    } else {
        None
    };

    let endTime = active_data.endTime;
    let endTime_since_start = if let Some(end) = endTime {
        if let Some(start) = startTime {
            (end - start).as_secs_f64()
        } else {
            None
        }
    } else {
        None
    };

    UseStateTransitionsTime {
        startTime,
        currentTime_since_start,
        endTime_since_start,
    }
}
```