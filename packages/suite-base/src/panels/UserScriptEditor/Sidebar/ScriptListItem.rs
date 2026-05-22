```rust
use web_sys::html_element::{Element, FocusEvent, KeyDownEvent};
use yew::{prelude::*};

#[derive(Properties, PartialEq)]
pub struct ScriptListItemProps {
    pub onClick: Callback<()>,
    pub onDelete: Callback<()>,
    pub onRename: Callback<String>,
    pub title: String,
    pub selected: Option<bool>,
}

#[function_component(ScriptListItem)]
pub fn script_list_item(props: &ScriptListItemProps) -> Html {
    let (label, set_label) = use_state(String::from);
    let (edit_mode, set_edit_mode) = use_state(false);

    let onChange = Callback::new(move |event| {
        let value = event.target().get_text_content();
        set_label(value);
    });

    let onDoubleClick = Callback::new(|| {
        set_edit_mode(true);
    });

    let onFocus = Callback::new(|_| {
        document().focus_on_element(&document().active_element().unwrap());
    });

    let onKeyDown = Callback::new(move |event| {
        if label.is_empty() {
            return;
        }
        if event.key() == "Escape" {
            set_label(props.title.clone());
            set_edit_mode(false);
        } else if event.key() == "Enter" {
            set_edit_mode(false);
            props.on_rename.emit(label.clone());
        }
    });

    let onBlur = Callback::new(|| {
        if label.is_empty() {
            return;
        }
        set_edit_mode(false);
        props.on_rename.emit(label.clone());

        if document().active_element().is_some() {
            document().active_element().unwrap().blur();
        }
    });

    let onButtonKeyDown = Callback::new(move |event| {
        if event.key() == "Enter" {
            set_edit_mode(true);
        }
    });

    html! {
        <li class={classes.list_item}>
            {if edit_mode {
                <>
                    <input type="text"
                           autoFocus
                           fullWidth
                           onBlur={onBlur}
                           onChange={onChange}
                           onFocus={onFocus}
                           onKeyDown={onKeyDown}
                           value={label.clone()}
                           class={classes.input}
                    />
                </>
            } else {
                <button class={classes.list_item_button}
                        selected={props.selected.is_some() && props.selected.unwrap()}
                        onClick={props.onClick}
                        onKeyDown={onButtonKeyDown}
                        onDoubleClick={onDoubleClick}>
                    {props.title.clone()}
                </button>
            }}
        </li>
    }
}

fn main() {
    yew::App::<ScriptListItem>::run();
}
```

O código Rust é uma versão funcional do script original TypeScript/React. Ele usa o Web_sys para interagir com a DOM, e usa o yew framework para criar um componente React com propriedades e estados.