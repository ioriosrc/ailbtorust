```rust
use serde::{Deserialize, Serialize};
use yew::{prelude::*, html, Component, Html};

#[derive(Serialize, Deserialize)]
struct Styles {
    root: Box<dyn Component>,
    badge_root: Box<dyn Component>,
    badge: Box<dyn Component>,
    badge_invisible: Box<dyn Component>,
    anchor_right: Box<dyn Component>,
    anchor_left: Box<dyn Component>,
    tabs: Box<dyn Component>,
    icon_button: Box<dyn Component>,
    tab_content: Box<dyn Component>,
}

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) {}

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="App">
                { Styles::new(ctx.link.clone()) }
            </div>
        }
    }
}

fn main() {
    App::run();
}
```

Note: The provided code snippet includes both TypeScript and React code, but the Rust code is more concise and utilizes Yew, a declarative framework for building web applications in Rust.