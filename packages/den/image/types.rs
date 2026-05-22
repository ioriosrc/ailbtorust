```rust
use std::collections::HashMap;

pub struct CameraModelsMap {
    models: HashMap<String, (String, Box<dyn CameraModelBuilder>)>,
}
```

Note that the `CameraModelBuilder` trait is not explicitly defined in Rust and needs to be imported or implemented separately.