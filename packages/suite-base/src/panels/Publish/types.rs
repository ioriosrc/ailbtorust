```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PublishConfig {
    topic_name: Option<String>,
    datatype: Option<String>,
    buttonText: Option<String>,
    button_tooltip: Option<String>,
    button_color: Option<String>,
    advanced_view: bool,
    value: Option<String>,
}
```