```rust
use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlDocument, Window};
use std::rc::Rc;

#[wasm_bindgen]
pub fn use_panning(canvas_div: &HtmlElement, coordinator: &Rc<dyn PlotCoordinator>, dragging_ref: Rc<web_sys::Ref<bool>>) {
    if canvas_div.is_none() || coordinator.is_none() {
        return;
    }

    let hammer_manager = Hammer::new(&canvas_div);

    let threshold = 10;

    hammer_manager.add(Hammer::Pan { threshold });

    hammer_manager
        .on("panstart", move |event| {
            *dragging_ref.borrow_mut() = true;
            let bounding_rect = canvas_div.getBoundingClientRect().unwrap();
            coordinator.send_interaction_event(
                InteractionEvent {
                    type: "panstart",
                    cancelable: false,
                    deltaY: event.deltaY(),
                    deltaX: event.deltaX(),
                    center: Position { x: event.center_x(), y: event.center_y() },
                    bounding_rect: Rect::new(bounding_rect.left(), bounding_rect.top(), bounding_rect.width(), bounding_rect.height()),
                }
            );
        })
        .on("panmove", move |event| {
            let bounding_rect = canvas_div.getBoundingClientRect().unwrap();
            coordinator.send_interaction_event(
                InteractionEvent {
                    type: "panmove",
                    cancelable: false,
                    deltaY: event.deltaY(),
                    deltaX: event.deltaX(),
                    center: Position { x: event.center_x(), y: event.center_y() },
                    bounding_rect: Rect::new(bounding_rect.left(), bounding_rect.top(), bounding_rect.width(), bounding_rect.height()),
                }
            );
        })
        .on("panend", move |event| {
            let bounding_rect = canvas_div.getBoundingClientRect().unwrap();
            coordinator.send_interaction_event(
                InteractionEvent {
                    type: "panend",
                    cancelable: false,
                    deltaY: event.deltaY(),
                    deltaX: event.deltaX(),
                    center: Position { x: event.center_x(), y: event.center_y() },
                    bounding_rect: Rect::new(bounding_rect.left(), bounding_rect.top(), bounding_rect.width(), bounding_rect.height()),
                }
            );

            // We need to do this a little bit later so that the onClick handler still sees
            // dragging_ref.current===true and can skip the seek.
            std::thread::sleep(std::time::Duration::from_millis(0));
            *dragging_ref.borrow_mut() = false;
        });

    // Remove hammer manager when component unmounts
    let _handle_remove = canvas_div.add_event_listener_with_callback("wheel", move |event| {
        if event.get_type().unwrap() == "wheel" && canvas_div.is_none() || coordinator.is_none() {
            return true;
        }
        
        let bounding_rect = canvas_div.getBoundingClientRect().unwrap();
        coordinator.send_interaction_event(
            InteractionEvent {
                type: "panend",
                cancelable: false,
                deltaY: event.deltaY(),
                deltaX: event.deltaX(),
                center: Position { x: event.center_x(), y: event.center_y() },
                bounding_rect: Rect::new(bounding_rect.left(), bounding_rect.top(), bounding_rect.width(), bounding_rect.height()),
            }
        );

        // We need to do this a little bit later so that the onClick handler still sees
        // dragging_ref.current===true and can skip the seek.
        std::thread::sleep(std::time::Duration::from_millis(0));
        *dragging_ref.borrow_mut() = false;
        
        true
    });

    handle_remove.forget();
}
```