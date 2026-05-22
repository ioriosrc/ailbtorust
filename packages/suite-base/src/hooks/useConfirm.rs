```rust
use std::rc::Rc;

use crate::{
    react::{create_ref, use_effect, use_keypress_event},
    ui::{Dialog, DialogContent, DialogTitle, DialogActions, Button},
};

type ConfirmVariant = "danger" | "primary";
type ConfirmAction = "ok" | "cancel";

type ConfirmOptions = {
    // the title of the confirm modal
    title: String,
    // text in the body of the confirm modal. Specify a string or JSX Element
    prompt: Option<String>,
    // the text for the OK button - defaults to "OK"
    ok: Option<String>,
    // the text for the cancel button - defaults to "Cancel"
    // set to false to completely hide the cancel button
    cancel: Option<String>,
    // indicate the type of confirmation
    variant: ConfirmVariant,
};

type ConfirmModalProps = ConfirmOptions + {
    onComplete: Rc<dyn Fn(ConfirmAction)>,
};

fn ConfirmModal(props: ConfirmModalProps) -> impl 'static {
    let original_on_complete = props.onComplete.clone();

    struct Completed;
    static mut COMPLETED: Option<Completed> = None;

    let completed = Rc::new(move || {
        if !COMPLETED.is_some() {
            COMPLETED = Some(Completed);
            original_on_complete(props.ok.unwrap_or("OK"));
        }
    });

    use_keypress_event!("Enter", move |_| {
        completed.clone()();
    });

    // Ensure we still call onComplete(undefined) when the component unmounts, if it hasn't been
    // called already
    use_effect(move || {
        let _ = Rc::clone(&props.onComplete);

        move || {
            completed.clone()();
        }
    }, []);

    let buttons = [
        props.cancel.is_some()
            .then(|| {
                Some(
                    <Button
                        variant="outlined"
                        color="inherit"
                        key="cancel"
                        onClick(move |_| {
                            completed.clone()();
                        })
                    >{props.cancel.unwrap_or("Cancel")}</Button>,
                )
            })
            .unwrap_or_default(),
        <Button
            key="confirm"
            variant=match props.variant {
                "danger" => "contained",
                _ => "primary",
            }
            color={props.variant == "danger" && props.ok.is_some() && props.cancel.is_none()
                .then(|| "error")
                .unwrap_or("primary")}
            type="submit"
        >
            {props.ok.unwrap_or("OK")}</Button>,
    ];
    if props.variant == "danger" {
        buttons.reverse();
    }

    move || (
        <Dialog
            open={true}
            onClose={move |_| {
                completed.clone()();
            }}
            maxWidth="xs"
            fullWidth
        >
            <form
                onSubmit={|_| {
                    event.preventDefault();
                    completed.clone()();
                }}
            >
                <DialogTitle>{props.title}</DialogTitle>
                {props.prompt.map(|prompt| (
                    <DialogContent key="prompt">{prompt}</DialogContent>
                ))}
                <DialogActions>{buttons}</DialogActions>
            </form>
        </Dialog>
    )
}

pub fn use_confirm() -> (Rc<dyn Fn(ConfirmAction)>, Option<impl 'static>) {
    let [modal, set_modal] = create_ref();

    let open_confirm = Rc::new(move |options| {
        modal().unwrap_or_default().clone()(|| {
            new Promise(|resolve| {
                set_modal(resolve);
            })
            .then(move |value| {
                value.unwrap_or("cancel");
            });
        });

        true
    });

    (open_confirm, modal())
}
```