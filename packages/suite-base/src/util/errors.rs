```rust
use std::error::Error;

#[derive(Debug)]
pub struct AppError {
    details: DetailsType,
    extra_info: Option<serde_json::Value>,
}

impl Error for AppError {}

impl AppError {
    pub fn new(details: DetailsType, extra_info: Option<serde_json::Value>) -> Self {
        let message = match details {
            DetailsType::Error(err) => err.stack().unwrap_or_else(|| err.to_string()),
            DetailsType::String(msg) => msg,
        };

        Self {
            details,
            extra_info,
            message,
        }
    }

    pub fn get_details(&self) -> &DetailsType {
        &self.details
    }

    pub fn get_extra_info(&self) -> Option<&serde_json::Value> {
        self.extra_info.as_ref()
    }
}
```