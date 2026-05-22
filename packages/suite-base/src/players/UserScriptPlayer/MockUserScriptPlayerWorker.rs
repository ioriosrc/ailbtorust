```rust
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct MockUserScriptPlayerWorker {
    pub port: Channel,
}

impl MockUserScriptPlayerWorker {
    pub fn new() -> Self {
        let (local, remote) = create_linked_channels();
        self.port = local;

        (local as { start?: () => void }).start = || {
            // no-op
        };
        let receiver = Rpc(remote);
        let receive_and_log = <fn(String, &dyn Fn(&[_]))> move |action, impl_| {
            receiver.receive(action, move |args| {
                validate_worker_args(args);
                self.messageSpy(action);
                let ret = impl_(args);
                validate_worker_args(ret);
                ret
            });
        };
        receive_and_log("generateRosLib", generate_ros_lib);
        receive_and_log("transform", transform);
        receive_and_log("registerScript", register_script);
        receive_and_log("processMessage", process_message);
    }

    // So tests can spy on what gets called
    pub fn messageSpy(&self, _action: String): () {
        // no-op
    }
}

fn validate_worker_args(arg: &serde::Value) {
    if !arg.is_null() && arg.as_object().is_some() {
        for (_, val) in arg.as_object().unwrap().iter() {
            validate_worker_args(val);
        }
    } else if let Some(array) = arg.as_array() {
        array.iter().for_each(validate_worker_args);
    }
}
```