```rust
use std::collections::HashMap;

// Define the type for application parameters as an enumeration
pub enum AppParameter {
    ApiKey(String),
    UserEmail(String),
}

// Define a context type to hold application parameters
pub struct AppParametersContext(HashMap<AppParameter, String>);

impl AppParametersContext {
    pub fn new(parameters: HashMap<AppParameter, String>) -> Self {
        Self(parameters)
    }

    pub fn get(&self, key: &AppParameter) -> Option<&String> {
        self.0.get(key)
    }
}

// Define the props for the AppParametersProvider component
#[derive(Properties, PartialEq)]
pub struct Props {
    children: Element<'static>,
    #[prop(default = HashMap::new())]
    app_parameters: HashMap<AppParameter, String>,
}

// Implement the Provider trait to provide the context to its descendants
impl Provider<AppParametersContext> for Props {
    fn provide(&self, ctx: &mut Context<'_>) -> Option<Self::Value> {
        Some(AppParametersContext::new(self.app_parameters.clone()))
    }
}
```