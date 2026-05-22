```rust
use std::convert::{TryFrom, TryInto};

#[derive(Debug)]
struct PlayerAlert {
    severity: String,
}

impl From<StatusLevel> for PlayerAlert {
    fn from(level: StatusLevel) -> Self {
        match level {
            StatusLevel::INFO => PlayerAlert { severity: "info".to_string() },
            StatusLevel::WARNING => PlayerAlert { severity: "warn".to_string() },
            StatusLevel::ERROR => PlayerAlert { severity: "error".to_string() },
        }
    }
}
```