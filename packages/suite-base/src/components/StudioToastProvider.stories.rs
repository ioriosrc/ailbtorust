```rust
use crate::components::{StudioToastProvider, use_snackbar};
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[allow(non_snake_case)]
    fn notify(negative: bool, severity: String, message: &str);
}

#[derive(Clone)]
pub struct ToastMessage {
    variant: Severity,
    message: String,
}

#[derive(Debug, Clone)]
pub enum Severity {
    Error,
    Warning,
    Info,
    Success,
}

impl ToastMessage {
    pub fn new(variant: Severity, message: String) -> Self {
        Self { variant, message }
    }

    #[allow(dead_code)]
    pub fn from_variant(variant: Severity, message: &str) -> Self {
        Self::new(variant, String::from(message))
    }

    pub fn to_string(&self) -> String {
        format!("{}: {}", self.variant.as_str(), self.message)
    }
}

pub struct StudioToastProviderComponent;

impl Component for StudioToastProviderComponent {
    type Context = ToastContext;
    type State = ();

    fn create_context() -> Self::Context {
        ToastContext {
            severity_queue: Vec::new(),
            message_queue: Vec::new(),
        }
    }

    fn render(self, ctx: &Self::Context) -> Html<Self> {
        html! {
            <StudioToastProvider context={ctx}>
                {for ctx.message_queue.iter().map(|message| html! { <div>{message}</div> })}
            </StudioToastProvider>
        }
    }

    fn update(&mut self, ctx: &mut Self::Context) {
        for message in ctx.severity_queue.drain(..) {
            notify(message.negative, message.severity.as_str(), message.message.as_str());
        }
    }
}

pub struct ToastContext;

impl Context<ToastProviderComponent> for ToastContext {
    fn inject(&self, child: Html<Self>) -> Html<Self> {
        html! { <div>{child}</div> }
    }

    fn on_message(&mut self, severity: Severity, message: &str) {
        let message = ToastMessage::from_variant(severity, message);
        self.message_queue.push(message);
    }
}
```