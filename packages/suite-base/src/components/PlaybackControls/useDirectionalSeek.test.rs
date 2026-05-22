```rust
use std::rc::Rc;

use crate::{
    broadcast::{MockBroadcastChannel, BroadcastChannel},
    ros_time::RosTime,
    use_app_configuration_value,
    test::builders::RosTimeBuilder,
};

pub fn use_directional_seek() -> (impl FnMut(), impl FnMut()) {
    let mock_broadcast_channel = Rc::new(MockBroadcastChannel);
    let (seek_forward_action, seek_backward_action) = use_app_configuration_value(|[forward_enabled, play_until_fn]| {
        if forward_enabled {
            move || {
                RosTimeBuilder.time().expect("Failed to create RosTime instance");
            }
        } else {
            move || {
                play_until_fn(RosTimeBuilder.time()).unwrap();
            }
        }
    });

    (seek_forward_action, seek_backward_action)
}
```