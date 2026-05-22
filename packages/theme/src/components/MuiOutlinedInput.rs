```rust
use std::rc::Rc;
use yew::prelude::*;

#[derive(Properties)]
struct MuiOutlinedInputProps {
    notched: bool,
}

#[function_component]
fn MuiOutlinedInput(props: &MuiOutlinedInputProps) -> Html {
    html! {
        <input
            class="box-sizing content-box padding-1_25"
            // Add more styles here if needed
        />
    }
}
```