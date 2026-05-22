```rust
use crate::log::Logger;
use std::{error::Error, future::Result};

type ResultFuture = Box<dyn Future<Output = Result<(), Error>>>;

fn log_error(err: &dyn Error) {
    log.error!("An error occurred: {}", err);
}

pub fn use_callback_with_toast<T>(
    callback: T,
    deps: &[&dyn std::any::Any],
) -> impl Fn(&mut Context<'_>, &T) + '_ {
    let enqueue_snackbar = use_context();
    
    Box::new(move |context, cb| {
        async move {
            match cb(context) {
                Ok(()) => Ok(()),
                Err(e) => {
                    log_error(&e);
                    
                    if e.is::<HttpError>() {
                        enqueue_snackbar("An HTTP error occurred".to_string());
                    } else {
                        let err_message = format!("{}", e);
                        enqueue_snackbar(err_message, "error");
                    }
    
                    Err(())
                }
            }
        }
    })
}

#[derive(Debug)]
struct HttpError {
    status_code: u16,
    message: String,
}

impl Error for HttpError {}

fn get_user_friendly_error_message(status_code: u16) -> String {
    // Implement logic to return user-friendly messages based on the HTTP status code
    match status_code {
        400 => "Bad request".to_string(),
        500 => "Internal server error".to_string(),
        _ => format!("Unexpected error with status code {}", status_code),
    }
}
```