```rust
use serde::{Deserialize, Serialize};
use serde_json::Value;

fn main() {
    // Given
    let filename = "example.bag";
    load_files(filename);

    // When
    click_on_right_sidebar_button();

    let new_variable_name_input = fill_global_variable_name();
    let new_variable_value_input = fill_global_variable_value("turtle1");

    play_button_click();

    // Then
    expect_no_topic_selected();

    // When
    let raw_messages_input_bar = fill_raw_messages_input("/tf.transforms[:]{child_frame_id==$globalVariable}");

    // Then
    expect_child_frame_id_turtle1();
}
```