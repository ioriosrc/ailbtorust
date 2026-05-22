```rust
use crate::components::panels::{PanelDragObject, SavedProps};
use crate::context::CurrentLayoutContext;
use crate::hooks::{getCurrentLayoutState, useDrag, useDropResult};
use crate::utils::MosaicDragType;

fn use_panel_drag(props: {
    tab_id: Option<String>,
    panel_id: Option<String>,
    on_drag_start: Option<impl Fn() + 'static>,
    on_drag_end: Option<impl Fn() + 'static>,
}) -> (ConnectDragSource, ConnectDragPreview) {
    let source_tab_id = props.tab_id;
    let panel_id = props.panel_id;
    let on_drag_start = props.on_drag_start;
    let on_drag_end = props.on_drag_end;

    // Retrieve necessary data from context
    let mosaic_window_actions = useContext(MosaicWindowContext);
    let mosaic_id = use_panel_mosaic_id();
    let { get_current_layout_state, start_drag, end_drag } = use_current_layout_actions();

    // Use React DnD hook to handle drag functionality
    let [connect_drag_source, connect_drag_preview] = use_drag::<PanelDragObject, MosaicDropResult, ()>(
        Box::new({
            type: MosaicDragType::WINDOW,
            item: move || {
                if on_drag_start.is_some() {
                    on_drag_start();
                }

                // Get current layout state and extract necessary information
                let selected_layout = get_current_layout_state()?;
                if selected_layout.data.is_none() {
                    return None;
                }

                let { layout, config_by_id } = selected_layout.data.unwrap();

                // The defer is necessary as the element must be present on start for HTML DnD to not cry
                let path = mosaic_window_actions.get_path();
                let deferred_hide = std::thread::spawn(move || {
                    start_drag(path, source_tab_id);
                });
                return PanelDragObject {
                    mosaic_id,
                    deferred_hide,
                    original_layout: layout,
                    original_config_by_id: config_by_id,
                };
            },
            end: move |item, monitor| {
                if on_drag_end.is_some() {
                    on_drag_end();
                }
                if item.original_layout.is_none() {
                    return;
                }

                // If the hide call hasn't happened yet, cancel it
                let thread_handle = item.deferred_hide.take().unwrap();
                thread_handle.join().unwrap();

                let own_path = mosaic_window_actions.get_path();
                let drop_result = monitor.get_drop_result::<MosaicDropResult>()?;
                let { position, path: destination_path, tab_id: target_tab_id } = drop_result;
                if panel_id.is_none() {
                    return;
                }

                end_drag({
                    original_layout: item.original_layout,
                    original_saved_props: item.original_config_by_id,
                    panel_id,
                    source_tab_id,
                    target_tab_id,
                    position,
                    destination_path,
                    own_path,
                });
            },
        }),
    );

    (connect_drag_source, connect_drag_preview)
}
```