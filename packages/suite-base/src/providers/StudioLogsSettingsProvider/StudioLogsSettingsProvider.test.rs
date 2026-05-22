```rust
use std::rc::Rc;

#[derive(Default)]
struct StudioLogsSettings {
    channels: Vec<LogChannel>,
    global_level: LogLevel,
}

impl StudioLogsSettings {
    fn set_global_level(&mut self, level: LogLevel) {
        self.global_level = level;
    }

    fn enable_channel(&mut self, name: &str) {
        if !self.channels.iter().any(|c| c.name == name) {
            self.channels.push(LogChannel::new(name));
        }
    }

    fn disable_channel(&mut self, name: &str) {
        if let Some(index) = self.channels.iter().position(|c| c.name == name) {
            self.channels.remove(index);
        }
    }

    fn enable_prefix(&mut self, name: &str, prefix: &str) {
        // Implement logic to handle channel prefixing
    }

    fn disable_prefix(&mut self, name: &str) {
        // Implement logic to handle channel prefix removal
    }
}

struct LogChannel {
    name: String,
    enabled: bool,
}

impl LogChannel {
    fn new(name: &str) -> Self {
        LogChannel {
            name: name.to_string(),
            enabled: true,
        }
    }
}

enum LogLevel {
    Info,
    Debug,
    Warn,
    Error,
}

fn main() {
    // Your Rust code here
}
```