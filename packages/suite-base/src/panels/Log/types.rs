```rust
use std::sync::Arc;
use std::cell::RefCell;

pub struct Config {
    pub search_terms: Vec<String>,
    pub min_log_level: i32,
    pub topic_to_render: Option<&str>,
    pub name_filter: RefCell<Option<NameFilter>>,
}

pub enum LogLevel {
    UNKNOWN = 0,
    DEBUG = 1,
    INFO = 2,
    WARN = 3,
    ERROR = 4,
    FATAL = 5,
}

pub type Ros1RosgraphMsgsLog = MessageEvent<Box<dyn Any>>;
pub type Ros2RosgraphMsgsLog = MessageEvent<Box<dyn Any>>;

pub type NormalizedLogMessage = {
    stamp: Time,
    level: LogLevel,
    message: String,
    name: Option<String>,
    file: Option<String>,
    line: Option<i32>,
};

pub type LogMessageEvent =
    MessageEvent<Ros1RosgraphMsgsLog>
    | MessageEvent<Ros2RosgraphMsgsLog>;

pub type LogListProps = {
    items: Arc<Vec<NormalizedLogMessage>>,
};
```