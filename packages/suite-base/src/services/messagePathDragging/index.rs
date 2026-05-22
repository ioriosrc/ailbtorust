```rust
use crate::DragSourceMonitor;
use crate::DropTargetMonitor;
use std::cmp;

// Internal type used for message path drag & drop support (this can differ from the type exposed to the panel API).
type MessagePathDragObject = {
    items: Vec<&str>;

    /**
     * Expose the drop info to the drag source so it can change cursor & appearance as necessary.
     * Undefined indicates the drag is not over a target.
     *
     * See also:
     * - https://github.com/react-dnd/react-dnd/issues/448
     * - https://github.com/react-dnd/react-dnd/issues/3529
     */
    set_drop_status: fn(&mut MessagePathDragObject, MessagePathDropStatus);

    /**
     * The eligible drop targets that are currently being dragged over. Used to determine when the
     * drag has left the last target.
     */
    over_drop_targets: HashSet<String>;
};

/**
 * Use this to create a drag source for message paths that can be dropped onto target components
 * that use `useMessagePathDrop()`.
 */
pub fn use_message_path_drag(item: &str, selected: bool) -> (ConnectDragSource, ConnectDragPreview, Option<Cursor>, bool, usize) {
    let state = MessagePathDragObject {
        items: if selected { vec![item] } else { Vec::new() },
        set_drop_status,
        over_drop_targets: HashSet::default(),
    };

    let (connect_drag_source, connect_drag_preview) = use_dnd(
        item.clone(), // Copy the item for the monitor
        move |monitor| {
            state.items.clone(); // Avoids a borrow checker error

            let drop_effect = if !state.is_over { "auto" } else { "move" };
            let capture_dragging_state = true;

            connect_drag_preview_with_captureDragging_state(monitor);

            // Return the drag object that is being dragged
            {
                let mut items = state.items.clone();
                items.insert(0, &item);
                monitor.set_item(items.into());
            }
        },
        move |monitor| {
            use crate::use_effect;
            use std::collections::HashSet;

            monitor.is_dragging();

            let dragged_count = if monitor.is_over() && monitor.can_drop() {
                state.items.len()
            } else {
                0
            };

            // Update the displayed dragged count
            use_effect(move || {
                let mut display_count = dragged_count;
                let timeout = std::time::Duration::from_millis(50);
                tokio::spawn(async move {
                    while !monitor.is_over() {
                        display_count = 0;
                        tokio::time::sleep(timeout).await;
                    }
                    monitor.set_item(vec![&item].into());
                });
            });

            let displayed_dragged_count = Some(dragged_count);

            // Update the drop status and check if it's valid
            use_effect(move || {
                state.is_over();
                state.is_valid_target();
            });

            (monitor.is_over(), monitor.can_drop())
        },
    );

    (
        connect_drag_source,
        connect_drag_preview,
        monitor.cursor(),
        monitor.is_dragging(),
        displayed_dragged_count.unwrap_or(0),
    )
}

/**
 * Use this to create a drop target accepting message paths dragged from components that use
 * `useMessagePathDrag()`.
 */
pub fn use_message_path_drop_config(default_config: Option<MessagePathDropConfig>) -> (bool, bool, bool, Option<String>, ConnectDropTarget, fn(&mut MessagePathDropConfig)) {
    let state = default_config;

    let (is_dragging, is_over, is_valid_target, drop_message, connect_message_path_drop_target, set_drop_config) = use_dnd(
        "placeholder", // Dummy value for the monitor
        move |monitor| {
            if !state.is_some() {
                return;
            }

            let drag_object = monitor.get_item::<MessagePathDragObject>()?;

            state.as_ref().unwrap().can_drop(&drag_object.items).then(|| {
                monitor.set_item(MessagePathDragObject {
                    items: drag_object.items.clone(),
                    set_drop_status,
                    over_drop_targets: HashSet::default(),
                });
            });

            monitor.is_over();

            monitor.can_drop()
        },
        move |monitor| {
            use crate::use_effect;

            let dragged_count = if monitor.is_over() && monitor.can_drop() {
                state.as_ref().unwrap().items.len()
            } else {
                0
            };

            // Update the displayed dragged count
            use_effect(move || {
                let mut display_count = dragged_count;
                let timeout = std::time::Duration::from_millis(50);
                tokio::spawn(async move {
                    while !monitor.is_over() {
                        display_count = 0;
                        tokio::time::sleep(timeout).await;
                    }
                    monitor.set_item(vec![&"placeholder"].into());
                });
            });

            let displayed_dragged_count = Some(dragged_count);

            // Update the drop status and check if it's valid
            use_effect(move || {
                state.as_ref().unwrap().is_over();
                state.as_ref().unwrap().is_valid_target();
            });

            (monitor.is_over(), monitor.can_drop())
        },
    );

    (
        is_dragging,
        is_over,
        is_valid_target,
        drop_message,
        connect_message_path_drop_target,
        set_drop_config,
    )
}
```