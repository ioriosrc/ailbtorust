```rust
use std::error::Error;
use std::fmt;

pub struct RosDb3IterableSource {
    files: Vec<String>,
}

impl RosDb3IterableSource {
    pub fn new(files: Vec<String>) -> Self {
        Self { files }
    }

    // Implement any necessary methods here
}

#[derive(Clone)]
pub struct WorkerSerializedIterableSourceWorker(RosDb3IterableSource);

impl Comlink::ComlinkObject for WorkerSerializedIterableSourceWorker {
    type Error = Box<dyn Error>;
    fn send(&self, message: impl Comlink::Sendable + 'static) -> Comlink::MessageResult<Self> {
        // Implement the logic to send a message and receive a result
        unimplemented!()
    }
}

fn initialize(args: Comlink::JsValue) -> WorkerSerializedIterableSourceWorker {
    let args = args.as_object().unwrap();
    let files = args.get("files").ok_or("Files required")?;
    let files: Vec<String> = serde_json::from_str(files.as_str().unwrap()).map_err(|_| "Invalid files format")?;

    if files.is_empty() {
        return Err(Box::new(std::io::ErrorKind::InvalidInput));
    }

    let source = RosDb3IterableSource::new(files);
    let wrapped = WorkerSerializedIterableSourceWorker(source);

    Comlink::proxy(wrapped)
}

Comlink::export(initialize, "initialize");
```

Note: This code is a simplified version and lacks the actual implementation of the `send` method in `Comlink::ComlinkObject`. You would need to add that logic based on your specific use case.