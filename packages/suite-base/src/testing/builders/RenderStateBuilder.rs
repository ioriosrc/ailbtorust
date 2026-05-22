```rust
use std::collections::HashMap;

struct RenderState {
    props: HashMap<String, String>,
}

impl RenderStateBuilder {
    pub fn render_state(props: Option<HashMap<String, String>>) -> RenderState {
        RenderState {
            props: props.unwrap_or_else(|| HashMap::new()),
        }
    }

    pub fn topic(props: Option<HashMap<String, String>>) -> Topic {
        Topic {
            name: BasicBuilder.string().to_string(),
            schema_name: BasicBuilder.string().to_string(),
        }
    }
}
```