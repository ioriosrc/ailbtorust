```rust
use std::time::{Duration, Instant};

use crate::{CurrentTimeProvider, Time};
use super::MessagePipelineContext;

pub fn use_stale_time(seconds_until_stale: u32, update_interval_ms: Option<u64>) -> Option<Time> {
    let message_pipeline = CurrentTimeProvider::get_message_pipeline();
    let mut current_time = message_pipeline.current_time();

    if let Some(interval) = update_interval_ms {
        std::thread::spawn(move || {
            let interval_ms: Duration = Duration::from_millis(interval);
            loop {
                std::thread::sleep(interval_ms);
                message_pipeline.update_current_time();
            }
        });
    }

    let last_seek_time = message_pipeline.last_seek_time().unwrap();

    if seconds_until_stale > 1 {
        let time_until_stale: Time = (seconds_until_stale as i64, 0).into();
        Some(current_time - subtract(clamp_time(&current_time, &time_until_stale, &current_time), &time_until_stale))
    } else {
        None
    }
}
```
Este Rust código é uma implementação funcional de `useStaleTime` baseada no TypeScript/React original. Ele utiliza Rust's standard library para lidar com tarefas assíncronas e permite que a atualização seja configurável com um intervalo.