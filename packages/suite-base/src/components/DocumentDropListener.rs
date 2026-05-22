```rust
use std::fs;
use std::path::Path;

fn is_supported_file_extension(filename: &str, allowed_extensions: &[&str]) -> bool {
    allowed_extensions.iter().any(|ext| filename.ends_with(ext))
}

#[derive(Debug)]
struct PendingFile {
    files: Vec<PathBuf>,
}

pub fn document_drop_listener(
    props: DocumentDropListenerProps,
) -> web_sys::HtmlElement {
    let (hovering, set_hovering) = web_sys::EventTarget::<web_sys::HtmlElement>::create_event_target_with_callback(
        "dragover",
        move |ev| {
            set_hovering(true);
            ev.prevent_default();
            ev.stopPropagation();
            ev.data_transfer().unwrap().drop_effect("copy");
        },
    );

    let (pending_files, set_pending_files) = web_sys::EventTarget::<web_sys::HtmlElement>::create_event_target_with_callback(
        "drop",
        move |ev| {
            if props.allowed_extensions.is_none() {
                return;
            }

            let data_transfer = ev.data_transfer().unwrap();
            let mut files: Vec<PathBuf> = data_transfer.files().map(|file| file.path()).collect();

            if is_supported_file_extension(&files[0].to_string_lossy(), props.allowed_extensions.as_ref()) {
                set_pending_files(Some(PendingFile { files }));
            } else {
                web_sys::window()
                    .with_unchecked_ref::<web_sys::Storage>()
                    .unwrap()
                    .set_item("not_supported", "true")
                    .unwrap();
            }

            ev.prevent_default();
            ev.stopPropagation();
        },
    );

    let (show_namespace_modal, set_show_namespace_modal) = web_sys::EventTarget::<web_sys::HtmlElement>::create_event_target_with_callback(
        "dragleave",
        move |_| {
            set_hovering(false);
        },
    );

    let on_drop_prop: Box<dyn Fn(&[PathBuf], Option<&str>) -> ()> = match props.on_drop {
        Some(f) => f.boxed(),
        None => Box::new(|_, _| {}),
    };

    web_sys::window().with_unchecked_ref::<web_sys::Storage>()
        .unwrap()
        .set_item("hovering", &hovering.to_string())
        .unwrap();

    let on_drag_over_prop = Box::new(move |_| {
        set_hovering(true);
        ev.prevent_default();
        ev.stopPropagation();
        ev.data_transfer().unwrap().drop_effect("copy");
    });

    web_sys::window().with_unchecked_ref::<web_sys::Storage>()
        .unwrap()
        .set_item("drag_over", &hovering.to_string())
        .unwrap();

    let on_drag_leave_prop = Box::new(move |_| {
        set_hovering(false);
    });

    web_sys::window().add_event_listener_with_callback(
        "dragover",
        move |ev| {
            ev.target()
                .unchecked_unwrap::<web_sys::HtmlElement>()
                .set_attribute("onmouseover", format!("return false;"));
            on_drag_over_prop.invoke(&ev).unwrap();
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "drop",
        move |ev| {
            ev.target()
                .unchecked_unwrap::<web_sys::HtmlElement>()
                .set_attribute("onmouseover", format!("return false;"));
            on_drop_prop.invoke(&ev).unwrap();
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "dragleave",
        move |ev| {
            ev.target()
                .unchecked_unwrap::<web_sys::HtmlElement>()
                .set_attribute("onmouseover", format!("return false;"));
            on_drag_leave_prop.invoke(&ev).unwrap();
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "input",
        move |ev| {
            let input = ev.target()
                .unchecked_unwrap::<web_sys::HtmlElement>()
                .unchecked_downcast::<web_sys::HTMLInputElement>();
            if !input.files().is_empty() {
                let files: Vec<PathBuf> = input.files().map(|file| file.path()).collect();

                if is_supported_file_extension(&files[0].to_string_lossy(), props.allowed_extensions.as_ref()) {
                    set_pending_files(Some(PendingFile { files }));
                } else {
                    web_sys::window()
                        .with_unchecked_ref::<web_sys::Storage>()
                        .unwrap()
                        .set_item("not_supported", "true")
                        .unwrap();
                }

                input.set_attribute("onmouseover", format!("return false;"));
            }
        },
    );

    let (pending_files_displayed, set_pending_files_displayed) = web_sys::EventTarget::<web_sys::HtmlElement>::create_event_target_with_callback(
        "click",
        move |_| {
            if pending_files.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    let (show_namespace_modal_displayed, set_show_namespace_modal_displayed) = web_sys::EventTarget::<web_sys::HtmlElement>::create_event_target_with_callback(
        "click",
        move |_| {
            if pending_files.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if show_namespace_modal_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if show_namespace_modal_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref()
                    .unwrap()
                    .invoke(&pending_files.as_ref().unwrap().files(), None)
                    .unwrap();
            } else {
                log::info!("No file to drop");
            }
        },
    );

    web_sys::window().add_event_listener_with_callback(
        "click",
        move |_| {
            if pending_files_displayed.is_some() {
                props.on_drop
                    .as_ref