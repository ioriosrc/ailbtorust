```rust
use console::{self, Level};

type LogLevel = &'static str;

struct Logger {
    level: Level,
}

impl Logger {
    pub fn new(level: &str) -> Self {
        Logger { level }
    }

    pub fn name(&self) -> String {
        format!("{}{}", self.level, self.name_impl())
    }

    fn name_impl(&self) -> String {
        // Implement logic to get the actual logger name
        "default".to_string() // Placeholder for actual implementation
    }

    pub fn is_level_on(&self, level: &str) -> bool {
        level.eq(self.level.as_ref())
    }

    pub fn get_level(&self) -> LogLevel {
        self.level.as_str()
    }

    pub fn set_level(&mut self, level: LogLevel) {
        self.level = match level {
            "debug" => Level::DEBUG,
            "info" => Level::INFO,
            "warn" => Level::WARN,
            "error" => Level::ERROR,
            _ => Level::WARN,
        };
    }

    pub fn debug(&self, args: impl std::fmt::Debug) {
        if self.is_level_on("debug") {
            console::log(Level::DEBUG, &args);
        }
    }

    pub fn info(&self, args: impl std::fmt::Debug) {
        if self.is_level_on("info") {
            console::log(Level::INFO, &args);
        }
    }

    pub fn warn(&self, args: impl std::fmt::Debug) {
        if self.is_level_on("warn") {
            console::log(Level::WARN, &args);
        }
    }

    pub fn error(&self, args: impl std::fmt::Debug) {
        if self.is_level_on("error") {
            console::log(Level::ERROR, &args);
        }
    }

    pub fn getLogger(&mut self, name: &str) -> Option<&'static Logger> {
        None // Implement logic to get the logger by name
    }

    pub fn channels(&self) -> Vec<&'static Logger> {
        Vec::new() // Implement logic to get all loggers
    }
}

fn to_level(level: String) -> LogLevel {
    match level.as_str() {
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::WARN,
    }
}

pub static DEFAULT_LOGGER: Logger = Logger { level: Level::WARN };

pub fn logger(name: &str) -> &'static Logger {
    // Implement logic to get the logger by name
    &DEFAULT_LOGGER // Placeholder for actual implementation
}

pub type LogLevel = &'static str;
```