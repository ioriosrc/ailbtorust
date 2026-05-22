```rust
mod react_hover_observer {
    use std::react::prelude::*;

    #[component]
    pub fn ReactHoverObserver(props: Props) -> Html {
        let (is_h hovering, mut set_is_h Hovering) = useState(false);

        if props.is_hovering {
            set_is_h Hovering(true);
        }

        html! {
            <div class={props.class_name}>
                {props.children({ is_hovering })}
            </div>
        }
    }
}
```