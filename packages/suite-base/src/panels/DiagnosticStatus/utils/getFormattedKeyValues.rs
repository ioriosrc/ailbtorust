```rust
use crate::suite_base::panels::DiagnosticStatus::{DiagnosticStatusMessage, KeyValue};
use std::regex::Regex;

const ALLOWED_TAGS: &[&str] = &["a", "b", "i", "em", "strong", "u"];

pub fn get_formatted_key_values(message: DiagnosticStatusMessage) -> Vec<FormattedKeyValue> {
    message.values
        .iter()
        .map(|{ key, value }: KeyValue| {
            let sanitize_key_html = HAS_ANY_HTML.test(key);
            let sanitized_value_html = HAS_ANY_HTML.test(value);

            FormattedKeyValue {
                key,
                key_html: if sanitize_key_html { sanitize(key) } else { None },
                value,
                value_html: if sanitized_value_html { sanitize(value) } else { None },
            }
        })
        .collect()
}

fn main() {
    // Example usage
    let diagnostic_status_message = DiagnosticStatusMessage {
        values: vec![
            KeyValue { key: "name".to_string(), value: "Alice".to_string() },
            KeyValue { key: "age".to_string(), value: "30".to_string() },
            KeyValue { key: "email".to_string(), value: "alice@example.com".to_string() },
        ],
    };

    let formatted_key_values = get_formatted_key_values(diagnostic_status_message);
    for kv in formatted_key_values {
        println!("Key: {}, Key HTML: {:?}, Value: {}, Value HTML: {:?}", kv.key, kv.key_html, kv.value, kv.value_html);
    }
}
```

Note that the `sanitize` function and the regex pattern (`HAS_ANY_HTML`) are assumed to be defined elsewhere in your Rust codebase.