```rust
use styled_components::{css, Theme};

pub fn use_file_types() -> (String, String) {
    let file_type_text = css! {
        margin_bottom: theme.spacing(2);
    };

    let file_names_text = css! {
        margin_bottom: theme.spacing(3);
        color: theme.palette.text.secondary;
    };

    let question_text = css! {
        margin_bottom: theme.spacing(2);
    };

    (file_type_text, file_names_text)
}
```