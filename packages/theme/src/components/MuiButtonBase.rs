```rust
use std::rc::Rc;
use yew::prelude::*;

// Define a custom override component for Material-UI ButtonBase
pub struct MuiButtonBase {}

impl Component for MuiButtonBase {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        MuiButtonBase {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html<Self> {
        html! {
            <button class="MuiButtonBase-root" disable_ripple=true>
                Button Text
            </button>
        }
    }
}
```