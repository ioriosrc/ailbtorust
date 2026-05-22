```rust
use std::rc::Rc;

// Define a trait for the context providers
pub trait ContextProvider {
    fn with_context<F>(&self, f: F) -> Self;
}

// Implement the ContextProvider trait for any struct that implements the Default trait
impl<T> ContextProvider for T
where
    T: Default + Clone,
{
    fn with_context<F>(&self, f: F) -> Self {
        let mut new_instance = Rc::new(self.clone());
        f(&mut new_instance);
        new_instance
    }
}

// The MultiProvider component in Rust
pub struct MultiProvider {
    providers: Vec<Rc<dyn ContextProvider>>,
}

impl MultiProvider {
    pub fn new(providers: Vec<Rc<dyn ContextProvider>>) -> Self {
        MultiProvider { providers }
    }

    pub fn render(&self) -> String {
        self.providers.iter().fold(String::new(), |acc, provider| {
            let wrapped_children = provider.with_context(|provider| {
                // Assuming each context provider returns a String
                format!("{}", provider)
            });
            acc + &wrapped_children
        })
    }
}
```