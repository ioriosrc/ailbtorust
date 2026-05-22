```rust
use std::error::Error;

type ReportErrorHandler = Box<dyn Fn(Error) + Send + Sync>;

const GLOBAL_WITH_HANDLER: &'static mut ReportErrorHandler = &mut || ();

/**
 * Report an error that has escaped past normal error-handling flows in the app and should be
 * triaged and diagnosed.
 */
pub fn report_error(error: Error): Result<(), Box<dyn Error>> {
    GLOBAL_WITH_HANDLER(&|e| e)?;
    Ok(())
}

/**
 * Set the handler function which will be called when an error is passed to `report_error()`. The default is
 * a no-op.
 */
pub fn set_report_error(handler: ReportErrorHandler) -> Result<(), Box<dyn Error>> {
    *GLOBAL_WITH_HANDLER = handler;
    Ok(())
}
```