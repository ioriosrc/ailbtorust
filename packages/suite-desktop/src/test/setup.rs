```rust
use std::sync::Arc;
use std::cell::RefCell;

// Intercepts console.error and console.warn calls to fail tests if they are called
// The user can indicate they expect the call to happen by checking the mock.calls
// and then clearing the mock via mockClear()
//
// We assign rather than spy to expose the mock for the user
pub struct ConsoleInterceptor {
    error_mock: Arc<RefCell<Vec<String>>>,
    warn_mock: Arc<RefCell<Vec<String>>>,
}

impl ConsoleInterceptor {
    pub fn new() -> Self {
        ConsoleInterceptor {
            error_mock: Arc::new(RefCell::new(Vec::new())),
            warn_mock: Arc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn log_error(&self, message: &str) {
        self.error_mock.borrow_mut().push(message.to_string());
    }

    pub fn log_warn(&self, message: &str) {
        self.warn_mock.borrow_mut().push(message.to_string());
    }

    pub fn clear_logs(&self) {
        self.error_mock.borrow_mut().clear();
        self.warn_mock.borrow_mut().clear();
    }
}

pub fn setup_console_interceptor() -> ConsoleInterceptor {
    let error_mock = Arc::new(RefCell::new(Vec::new()));
    let warn_mock = Arc::new(RefCell::new(Vec::new()));

    let interceptor = ConsoleInterceptor { error_mock, warn_mock };

    // Override the console.error and console.warn functions to use our interceptor
    console_error!(|e| interceptor.log_error(e));
    console_warn!(|w| interceptor.log_warn(w));

    interceptor
}
```

### Explanation:
1. **ConsoleInterceptor struct**:
   - `error_mock` and `warn_mock`: These are reference-counted mutable vectors that will hold the logs of `console.error` and `console.warn` calls.
   - `log_error` and `log_warn`: These methods append the log messages to their respective vectors.

2. **clear_logs**: This method clears both vectors, effectively resetting the mock.

3. **setup_console_interceptor**:
   - Creates an instance of `ConsoleInterceptor`.
   - Overrides the original `console.error` and `console.warn` functions with our custom implementations that append logs to the mock.
   - Returns the configured interceptor for further use in tests.

This setup allows you to intercept and monitor console errors and warnings, providing a way to verify their behavior during tests.