```rust
pub const HTTP_ERRORS: &[(&str, &str)] = &[
    ("network_error", "Network connection error. Please check your connection."),
    ("bad_request", "Invalid request. Please check your input and try again."),
    ("unauthorized", "You are not authenticated."),
    ("forbidden", "You do not have permission to perform this action."),
    ("not_found", "The requested resource was not found."),
    ("conflict", "The resource already exists or has been modified."),
    ("internal_server_error", "Server error. Please try again later."),
    ("client_error", "Request error. Please check your input and try again."),
    ("server_error", "Server error. Please try again later."),
];
```