```rust
use yew::prelude::*;

struct Props {
    is_chrome: bool,
    current_version: u32,
    is_dismissable: bool,
}

#[function_component(CompatibilityBanner)]
pub fn compatibility_banner(props: &Props) -> Html {
    let mut class = classes!("banner", "compatibility-banner");

    if props.is_dismissable {
        class.push("dismissable");
    }

    html! {
        <div class={class}>
            {if props.is_chrome {
                html! {
                    <p>Chrome version {props.current_version}</p>
                }
            } else {
                html! {
                    <p>Unsupported browser</p>
                }
            }}
        </div>
    }
}

fn main() {
    yew::App::<Props>::builder().build();
}
```