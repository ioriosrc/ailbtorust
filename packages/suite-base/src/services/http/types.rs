```rust
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct HttpRequestOptions {
    timeout: Option<Duration>,
    response_type: Option<&'static str>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HttpResponse<T> {
    data: T,
    timestamp: String,
    path: String,
}

type DetailErrorApiResponse = {
    field: String,
    constraints: std::collections::HashMap<String, String>,
};

type ErrorResponse = {
    status_code: u16,
    message: String,
    error: String,
    details: Vec<DetailErrorApiResponse>,
};

#[derive(Debug, Deserialize)]
pub enum HttpStatus {
    OK = 200,
    CREATED = 201,
    NO_CONTENT = 204,
    BAD_REQUEST = 400,
    UNAUTHORIZED = 401,
    FORBIDDEN = 403,
    NOT_FOUND = 404,
    CONFLICT = 409,
    INTERNAL_SERVER_ERROR = 500,
}
```