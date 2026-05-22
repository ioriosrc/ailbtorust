```rust
use log::{self, LogLevel};

type Logger = Box<dyn Fn(LogLevel)>;

struct StudioLogConfigChannel {
    name: String,
    enabled: bool,
}

struct StudioLogsSettings {
    global_level: LogLevel,
    channels: Vec<StudioLogConfigChannel>,
}

fn to_log_level(level: &str) -> LogLevel {
    match level.to_lowercase().as_str() {
        "debug" => LogLevel::Debug,
        "info" => LogLevel::Info,
        "warn" => LogLevel::Warn,
        "error" => LogLevel::Error,
        _ => LogLevel::Off,
    }
}

fn create_studio_logs_settings_store(
    initial_state: Option<&LocalStorageSaveState>,
) -> Box<dyn FnMut(&mut StudioLogsSettings, &mut LocalStorageSaveState)> {
    let global_level = match initial_state {
        Some(state) => to_log_level(&state.global_level),
        None => LogLevel::Warn,
    };

    let disabled_channels = initial_state.map(|state| state.disabled_channels).unwrap_or_default();

    log::info!("Initializing log Config. {} disabled channels.", disabled_channels.len());

    let mut channels: Vec<StudioLogConfigChannel> = vec![];
    let channel_by_name = std::collections::HashMap::new();
    let sorted_channels = log::channels().sort_unstable();

    for &channel in sorted_channels {
        let name = channel.name().map(|name| name.to_string());
        if disabled_channels.contains(&name.as_str()) {
            channel.set_level(LogLevel::Warn);
        } else {
            channel.set_level(global_level);
        }

        if !channel_by_name.contains_key(&name) {
            channels.push(StudioLogConfigChannel {
                name,
                enabled: channel.is_level_on(global_level),
            });
        }

        let existing = channel_by_name.entry(name).or_insert_with(|| vec![]);
        existing.push(channel);
    }

    let regenerate_channels = move |get: &mut StudioLogsSettings, set: &mut LocalStorageSaveState| {
        let current_global_level = get.global_level;
        let mut did_change = false;
        for channel in channels.iter_mut() {
            if let Some(log_channels) = channel_by_name.get(&channel.name) {
                if !log_channels[0].is_level_on(current_global_level) == channel.enabled {
                    channel.enabled = !channel.enabled;
                    did_change = true;
                }
            }
        }

        if !did_change {
            return;
        }

        set.channels(channels);
    };

    Box::new(move |set, state| {
        set.global_level(global_level);

        for &name in disabled_channels.iter() {
            let log_channels = channel_by_name.get_mut(&name).unwrap();
            log_channels.iter().for_each(|channel| channel.set_level(LogLevel::Warn));
        }

        regenerate_channels(set, state);
    })
}
```