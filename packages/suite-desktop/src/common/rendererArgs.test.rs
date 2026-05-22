```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct RendererArg {
    key: String,
    value: Vec<String>,
}

fn encode_renderer_arg(key: &str, values: &[String]) -> String {
    let encoded_values = values.iter().map(|value| value.to_string()).collect::<Vec<_>>();
    format!("--{}={}", key, encoded_values.join(","))
}

fn decode_renderer_arg(key: &str, args: &[RendererArg]) -> Vec<String> {
    args
        .iter()
        .filter(|arg| arg.key == key)
        .map(|arg| arg.value.clone())
        .flatten()
        .collect::<Vec<_>>()
}
```