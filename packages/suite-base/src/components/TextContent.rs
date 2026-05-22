```rust
use yew::prelude::*;

#[derive(PartialEq, Eq, Debug)]
pub struct TextContentProps {
    pub style: Option<css::Style>,
    pub allow_markdown_html: bool,
}

impl Component for TextContent {
    type Message = ();

    fn create(ctx: &Context<Self>) -> Self {
        TextContent { ..Default::default() }
    }

    fn update(&mut self, msg: Self::Message, ctx: &Context<Self>) {
        // Update logic if needed
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let style = match self.props.style {
            Some(style) => style,
            None => css::Style::default(),
        };

        html! {
            <div class={classes!([
                "text-content",
                if self.props.allow_markdown_html { "markdown-html" },
            ])} style={style}>
                {if let Some(children) = &self.props.children {
                    match children.as_ref() {
                        Html::Text(text) => html! { <p>{text}</p> },
                        Html::Node(node) => node.render(ctx),
                        _ => html! {},
                    }
                } else {
                    html! {};
                }}
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>) {
        // Rendering logic if needed
    }
}

fn main() {
    App::<TextContent>::run();
}
```