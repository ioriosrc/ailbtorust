```rust
use std::error;
use std::fmt;

#[derive(Debug)]
pub struct HttpError {
    pub message: String,
    pub status: u16,
    pub status_text: String,
    pub response: Option<Response>,
}

impl Error for HttpError {}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({})",
            self.status_text,
            if let Some(response) = &self.response {
                format!("Response: {}", response)
            } else {
                String::from("No response")
            }
        )
    }
}

impl HttpError {
    pub fn new(message: String, status: u16, status_text: String) -> Self {
        HttpError {
            message,
            status,
            status_text,
            response: None,
        }
    }

    pub fn with_response(response: Response) -> Self {
        HttpError {
            message: BasicBuilder.string(),
            status: BasicBuilder.u16(),
            status_text: BasicBuilder.string(),
            response: Some(response),
        }
    }

    pub fn get_user_friendly_error_message(&self) -> String {
        if self.status == 0 {
            "Network connection error. Please check your connection.".to_string()
        } else {
            format!(
                "Request error. Please check your input and try again.",
                self.status_text
            )
        }
    }

    pub fn user_friendly_error_message(&self) -> Option<String> {
        if let Some(response) = &self.response {
            Some(format!("Response: {}", response))
        } else {
            None
        }
    }

    pub fn is_network_error(&self) -> bool {
        self.status == 0
    }

    pub fn is_client_error(&self) -> bool {
        self.status >= 400 && self.status < 500
    }

    pub fn is_server_error(&self) -> bool {
        self.status >= 500 && self.status < 600
    }
}
```