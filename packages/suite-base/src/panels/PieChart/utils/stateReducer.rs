```rust
use crate::types::{PieChartState, PieChartAction};
use std::time::Duration;

pub fn state_reducer(state: &mut PieChartState, action: PieChartAction) {
    match action.type {
        PieChartAction::Frame => {
            *state = handle_frame(*state, action);
        }
        PieChartAction::Path => {
            *state = handle_path(*state, action);
        }
        PieChartAction::Seek => {
            state.seek(Duration::from_secs(action.payload));
        }
        _ => {}
    }
}
```