```rust
use super::*;
use crate::base::message_converters::InstalledMessageConverter;

fn build_contribution_points(extension_info: ExtensionInfo, extension_source: &str) -> ContributionPoints {
    let mut panels = HashMap::new();
    let mut message_converters = Vec::new();
    let mut topic_alias_functions = vec![];

    // Assuming globalThis is replaced with something that can store these values
    global_this.panels = panels;
    global_this.message_converters = message_converters;
    global_this.topic_alias_functions = topic_alias_functions;

    // Execute the extension source to register panels, message converters, and topic aliases
    eval_extension_source(extension_source);

    // Clean up
    global_this.panels.clear();
    global_this.message_converters.clear();
    global_this.topic_alias_functions.clear();

    ContributionPoints {
        panels,
        message_converters,
        topic_alias_functions,
    }
}
```