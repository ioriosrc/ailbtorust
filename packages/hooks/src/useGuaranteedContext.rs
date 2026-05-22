```rust
use std::rc::Rc;

pub struct UseGuaranteedContext<T> {
    context_type: Rc<dyn std::any::Any>,
}

impl<T> UseGuaranteedContext<T> {
    pub fn new(context_type: Rc<dyn std::any::Any>) -> Self {
        UseGuaranteedContext { context_type }
    }

    pub fn use(&self) -> &T {
        let context = self.context_type.as_ref();
        if let Some(context) = context.downcast_ref::<T>() {
            context
        } else {
            panic!(
                "useGuaranteedContext got null for contextType: '{}'",
                context_type.type_name(),
            );
        }
    }
}
```