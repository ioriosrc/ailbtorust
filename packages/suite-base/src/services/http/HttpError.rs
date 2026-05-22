```rust
use std::error::Error;

#[derive(Debug, PartialEq)]
pub struct HttpError {
    status: u16,
    status_text: String,
    response: Option<Response>,
}

impl HttpError {
    pub fn new(message: &str, status: u16, status_text: &str, response: Option<Response>) -> Self {
        HttpError {
            message: message.to_string(),
            status,
            status_text: status_text.to_string(),
            response,
        }
    }

    pub fn user_friendly_error_message(&self) -> String {
        let t = shared_i18n_object.t;

        if self.status == 0 {
            return t("httpErrors:networkError");
        }

        match self.status {
            HttpStatus::BAD_REQUEST => t("httpErrors:badRequest"),
            HttpStatus::UNAUTHORIZED => t("httpErrors:unauthorized"),
            HttpStatus::FORBIDDEN => t("httpErrors:forbidden"),
            HttpStatus::NOT_FOUND => t("httpErrors:notFound"),
            HttpStatus::CONFLICT => t("httpErrors:conflict"),
            HttpStatus::INTERNAL_SERVER_ERROR => t("httpErrors:internalServerError"),
            _ if self.status >= 400 && self.status < 500 => {
                t("httpErrors:clientError")
            }
            _ if self.status >= 500 => t("httpErrors:serverError"),
            _ => self.message.clone(),
        }
    }
}

impl Error for HttpError {}
```