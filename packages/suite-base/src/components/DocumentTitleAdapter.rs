```rust
use crate::components::message_pipeline::{MessagePipelineContext, use_message_pipeline};
use crate::utils::select_player_name;

#[function_component]
pub fn DocumentTitleAdapter() -> JSX.Element {
    let player_name = use_message_pipeline(select_player_name);

    useEffect(
        move || {
            if !player_name.is_empty() {
                if navigator.userAgent.contains("Mac") {
                    window.document.title = &player_name;
                } else {
                    window.document.title = format!("{} – Lichtblick", &player_name);
                }
            } else {
                window.document.title = "Lichtblick";
            }

            Box::pin(async {})
        },
        &[&player_name],
    );

    <div></div>
}
```