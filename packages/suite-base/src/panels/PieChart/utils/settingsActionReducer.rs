```rust
use immer::Immer;

fn settings_action_reducer(
    mut prev_config: Immer<PieChartConfig>,
    action: SettingsTreeAction,
) -> Immer<PieChartConfig> {
    match action.action {
        SettingsTreeAction::PerformNodeAction { id } => {
            panic!("Unhandled node action: {}", id);
        }
        SettingsTreeAction::Update { path, value } => {
            let mut current_path = &mut prev_config;
            for segment in path.iter() {
                if *segment == "general" {
                    // Navigate into the "general" section
                    break;
                }
                current_path = &mut current_path[*segment];
            }
            if let Some(last_segment) = path.last() {
                // Update the last segment of the path
                current_path[last_segment] = value;
            }
        }
    }
    prev_config
}
```