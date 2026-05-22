```rust
use std::time::{Duration, Instant};

type MessageTypeByTopic = HashMap<String, MessageDefinition>;
type MessageTypeBySchemaName = HashMap<String, Box<dyn Any + Send>>;

pub type Message<T> = &'static dyn Any + Send;
pub type Input<T> = (&'static str, Time, Message<T>);
pub type RGBA = (f32, f32, f32, f32);
pub type Header = (String, Instant, u32);
pub type Point = (f32, f32, f32);
pub type Time = (i64, i32);
pub type Translation = (f32, f32, f32);
pub type Rotation = (f32, f32, f32, f32);
pub type Pose = (Point, Quaternion);
pub type Quaternion = (f32, f32, f32, f32);
pub type Transform = (Header, String, Translation, Rotation);
```