```rust
use std::error::Error;
use serde::{Deserialize, Serialize};
use crate::validation_error_to_string;

type Value = serde_json::Value;
type OnChange = fn(Value) -> ();

struct ValidatedInput {
    dataTestId: Option<String>,
    dataValidator: Box<dyn Fn(&Value) -> ValidationResult>,
    onChange: Option<OnChange>,
    parse: fn(&str) -> Result<Value, Box<dyn Error>>,
    readOnly: bool,
    maxHeight: Option<u32>,
    value: Value,
}

impl ValidatedInput {
    fn new(dataTestId: Option<String>, dataValidator: impl Fn(&Value) -> ValidationResult + 'static, onChange: Option<OnChange>, parse: fn(&str) -> Result<Value, Box<dyn Error>>, readOnly: bool, maxHeight: Option<u32>) -> Self {
        Self {
            dataTestId,
            dataValidator: Box::new(data_validator),
            onChange,
            parse,
            readOnly,
            maxHeight,
            value: serde_json::to_value(props.value).unwrap(),
        }
    }

    fn change(&mut self, new_val: Value) {
        if !self.read_only {
            let validation_result = (self.data_validator)(&new_val);
            match validation_result {
                Ok(_) => self.set_error(""), // clear the previous error
                Err(err) => self.set_error(error.to_string()),
            }
            if self.onChange.is_some() {
                (self.onChange)(new_val);
            }
        }
    }

    fn set_error(&mut self, error: String) {
        self.error = error;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Example usage
    let props = serde_json::from_str("{\"key\":\"value\"}").unwrap();
    let validated_input = ValidatedInput::new(
        Some("validated-input-example".to_string()),
        |val| validate_value(val),
        None,
        |s| serde_json::from_str(s).map_err(|e| e.to_string()),
        false,
        None,
    );

    validated_input.change(serde_json::to_value("{\"other_key\":\"other_value\"}").unwrap());

    Ok(())
}
```