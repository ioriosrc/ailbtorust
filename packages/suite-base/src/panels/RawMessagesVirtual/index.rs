```rust
use suite_base::components::Panel;
use suite_base::panels::raw_messages_virtual::RawMessagesVirtual;
use suite_base::panels::raw_messages_virtual::constants::*;

pub fn main() {
    Panel::new(
        RawMessagesVirtual::new(Object::from({
            let mut obj = HashMap::new();
            obj.insert("panelType", "RawMessagesVirtual");
            obj.insert("defaultConfig", RAW_MESSAGES_VIRTUAL_DEFAULT_CONFIG);
            obj
        })),
    );
}
```
Note: This is a simplified Rust version of the TypeScript code. It uses `HashMap` for object creation, and the `Panel` and `RawMessagesVirtual` structs are assumed to be defined in a similar manner as in TypeScript, with necessary imports and definitions. The actual structure and methods of these components would need to be filled out based on their TypeScript counterparts.