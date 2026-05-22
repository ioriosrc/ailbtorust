```rust
use std::any::{Any, TypeId};
use crate::{
    messages::{BaseMarker, InstancedLineListMarker},
    types::{MouseEventObject, SelectedObject},
};

type ClickedPosition = (f64, f64);

#[derive(Debug)]
struct InteractionContextMenuItem {
    interactive_object: Option<SelectedObject>,
}

impl InteractionContextMenuItem {
    fn new(interactive_object: Option<SelectedObject>) -> Self {
        Self { interactive_object }
    }

    fn select_item_object(&self) -> impl Fn() + 'static {
        move || self.select_object(&self.interactive_object)
    }
}

#[derive(Debug)]
struct InteractionContextMenuProps {
    clicked_objects: Vec<MouseEventObject>,
    clicked_position: ClickedPosition,
    onClose: fn(),
    select_object: fn(Option<&SelectedObject>),
}

fn get_instance_obj(marker: &Any, idx: usize) -> Option<&dyn BaseMarker> {
    if let Some(instanced_line_list_marker) = marker.downcast_ref::<InstancedLineListMarker>() {
        return instanced_line_list_marker.metadata_by_index.get(idx);
    }
    None
}

fn getObject(selected_object: Option<MouseEventObject>) -> Option<&dyn Any> {
    match selected_object {
        Some(interactive_object) => {
            if let Some(instanced_line_list_marker) = interactive_object.object.downcast_ref::<InstancedLineListMarker>() {
                if let Some(metadata_by_index) = instanced_line_list_marker.metadata_by_index {
                    return metadata_by_index.get(interactive_object.instance_index);
                }
            }
        },
        None => None,
    }
}

fn main() {
    // Example usage
}
```