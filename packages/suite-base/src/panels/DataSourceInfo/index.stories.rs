```rust
use bevy::prelude::*;
use bevy_ros2_bridge::{Client, ClientOptions, RosClient};
use lighthouse_presentation_core::models::DataSourceInfoModel;
use lighthouse_presentation_core::services::DataSourceInfoService;

fn main() {
    let client = Client::<RosClient>::new("localhost:16000");
    if !client.is_connected() {
        panic!("Failed to connect to ROS 2 server!");
    }

    let service = DataSourceInfoService::new(client);
    let model = service.get_data();

    println!("{:#?}", model);
}
```