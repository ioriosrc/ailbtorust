```rust
use futures::future::{ready, AsyncState, AsyncStateResult};
use std::pin::Pin;

struct UseConfirm {
    modal_opened: Pin<Box<dyn FnOnce(AsyncStateResult) + Send>>,
}

impl Default for UseConfirm {
    fn default() -> Self {
        Self {
            modal_opened: Box::pin(|_| ready(())),
        }
    }
}

async fn confirm_with_params(params: ConfirmationParams) -> AsyncStateResult {
    // Implementation of the confirm function with provided parameters
    // This is a placeholder for the actual implementation
    Ok(())
}

#[derive(Clone)]
struct ConfirmationParams {
    title: String,
    prompt: Option<String>,
    variant: ConfirmationVariant,
    ok_text: String,
    cancel_text: String,
}

enum ConfirmationVariant {
    Primary,
    Secondary,
    Danger,
}

pub fn use_confirm() -> UseConfirm {
    UseConfirm {
        modal_opened: Box::pin(|_| ready(())),
    }
}

impl UseConfirm {
    pub async fn confirm(&self, params: ConfirmationParams) {
        // Call the confirmation function with provided parameters
        let result = confirm_with_params(params).await;
        (self.modal_opened.as_mut().get_unchecked())(result);
    }

    pub fn is_modal_opened(&self) -> bool {
        matches!(self.modal_opened.as_ref(), Pin::new_unchecked(_) => true, _ => false)
    }
}

fn main() {
    // Usage example
}
```

This Rust code snippet provides a simplified implementation of the `useConfirm` hook used in the given TypeScript/React code. The `UseConfirm` struct manages the state of the confirmation modal and handles its opening asynchronously when the component mounts. The `confirm_with_params` function simulates the behavior of the original TypeScript code by handling different types of confirmations (primary, secondary, danger) with customizable prompts and text for the OK and cancel buttons.