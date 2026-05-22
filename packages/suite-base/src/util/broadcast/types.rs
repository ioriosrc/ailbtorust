```rust
use std::collections::HashSet;

type BroadcastMessageEvent = (String, i64);

pub type ChannelListeners = HashSet<Box<dyn Fn(BroadcastMessageEvent) + Send>>;
```

Note: The TypeScript code includes an import statement for a package not available in Rust. In Rust, there's no equivalent `BroadcastMessageEvent` type. Additionally, the use of `Time` from "@lichtblick/suite" is also absent in Rust as it doesn't have a direct equivalent type or standard library feature.

The `ChannelListeners` type in TypeScript is implemented using a set to allow for easy addition and removal of listeners. In Rust, we can achieve similar functionality with a `HashSet` where the elements are boxed functions (`Box<dyn Fn(BroadcastMessageEvent) + Send>`).