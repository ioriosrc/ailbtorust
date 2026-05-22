```rust
use mui::components::Tooltip;
use mui::utils::Tag;
use react::prelude::*;

#[derive(PartialEq)]
pub struct PropsMaybeCollapsedValue {
    pub item_label: String,
}

fn main() -> Html {
    let (item_label, showing_entire_label) = React.useState(() => {
        let original_label = "This is a very long label that exceeds the maximum allowed length.";
        if original_label.len() >= COLLAPSE_TEXT_OVER_LENGTH {
            (original_label.to_string(), true)
        } else {
            (original_label.to_string(), false)
        }
    });

    let expand_text = React.useCallback(move || {
        set_showing_entire_label(true);
    }, []);

    let truncated_item_text = if showing_entire_label {
        item_label.clone()
    } else {
        &item_label[..COLLAPSE_TEXT_OVER_LENGTH]
    };

    html! {
        <Tooltip title={showing_entire_label.then(|| "")} placement="top">
            <Tag
                onClick={expand_text}
                onKeyDown={(e) => e.key == "Enter" || e.key == " " { e.preventDefault(); expand_text() }}
                tabIndex={showing_entire_label.then(|| -1)}
                style={{ cursor: showing_entire_label.then(|| "inherit") }}
                aria-expanded={showing_entire_label}
            >
                {format!("{}{}", truncated_item_text, if showing_entire_label {""} else {"…"})}
            </Tag>
        </Tooltip>
    }
}
```