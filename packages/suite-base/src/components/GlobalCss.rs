```rust
use std::rc::Rc;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GlobalStylesProps {
    pub theme: Rc<Theme>,
}

#[function_component(GlobalCss)]
fn GlobalCss(props: &GlobalStylesProps) -> Html {
    html! {
        <style>
            html,
            body {
                box-sizing: border-box;
                margin: 0;
                padding: 0;
                height: 100%;
                width: 100%,

                /* https://github.com/necolas/normalize.css/blob/master/normalize.css#L12 */
                line-height: 1.15;
            }

            *,
            *:before,
            *:after {
                box-sizing: inherit;
            }

            body {
                background: props.theme.background.default;
                color: props.theme.text.primary;
                font: inherit;
                fontFamily: props.theme.typography.body2.fontFamily;
                font-feature-settings: props.theme.typography.body2.fontFeatureSettings;
                fontSize: props.theme.typography.body2.fontSize;
                fontWeight: props.theme.typography.body2.fontWeight,

                /* Prevent scroll "bouncing" since the app workspace is not scrollable. Allows individual
                scrollable elements to be scrolled without the whole page moving (even if they don't
                preventDefault on scroll events). */
                overscroll_behavior: none;
                overflow: hidden;
            }

            #root {
                height: 100%;
                width: 100%;
                display: flex;
                flexDirection: column;
                position: relative;
                flex: 1 1 100%;
                outline: none;
                overflow: hidden;
                zIndex: 0;
            }
        </style>
    }
}
```