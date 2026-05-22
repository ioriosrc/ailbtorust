```rust
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::components::AppBar::APP_BAR_HEIGHT;
use crate::icons::{CheckmarkCircle20Regular, DismissCircle20Regular, Dismiss16Filled, Info20Regular, Warning20Regular};

use mui::{
  components::{SnackbarProvider, SnackbarKey},
  material_ui_core as ui,
  use_mutable_ref,
};
use notistack::{Notificator, NotifType, StackProps};

mod styles;
pub use styles::use_styles;

const ICON_SIZE = 16;

pub struct StudioToastProvider {
    children: Arc<Mutex<dyn Fn() -> ReactNode>>,
}

impl StudioToastProvider {
    pub fn new(children: impl Fn() -> ReactNode) -> Self {
        Self {
            children: Arc::new(Mutex::new(children)),
        }
    }

    pub fn render(&self, children: Box<dyn Fn() -> ReactNode>) {
        let notificator = Notificator::builder()
            .max_stack(5)
            .prevent_duplicate(true)
            .timeout(Duration::from_secs_f32(5.0))
            .build();

        let child = children;
        thread::spawn(move || {
            while true {
                let notification = child();
                notificator.send_notification(notification, NotifType::Custom);
                thread::sleep(Duration::from_secs_f32(1.0));
            }
        });
    }

    pub fn update(&self, children: Box<dyn Fn() -> ReactNode>) {
        *self.children.lock().unwrap() = children;
    }
}
```