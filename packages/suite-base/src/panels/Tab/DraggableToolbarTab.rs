```rust
use react::prelude::*;
use react_dnd::{use_drag, use_drop};

type Props = {
    isActive: bool;
    panelId: String;
    actions: TabActions;
    tabCount: i32;
    tabIndex: i32;
    tabTitle: String;
};

fn DraggableToolbarTab(props: Props) -> ReactElement {
    let { isActive, tabCount, actions, panelId, tabTitle, tabIndex } = props;

    let ({ isDragging }, connect_drag_ref) = use_drag::<TabLocation, _, { isDragging: bool }>({
        type: "TAB_DRAG_TYPE",
        item: { panel_id: &panelId, tab_index: &tabIndex },
        collect: |monitor| ({
            isDragging: monitor.isDragging(),
        }),
    });

    let ({ highlight }, connect_drop_ref) = use_drop::<TabLocation, _, { highlight: Option<&str> }>({
        accept: "TAB_DRAG_TYPE",
        collect: |monitor| ({
            highlight: monitor.isOver().map(|item| {
                if item.tab_index! < tabIndex { "after" } else { "before" }
            }),
        }),
        drop: move_tab(move |source, target| {
            let source = TabLocation {
                panel_id: &source.panel_id,
                tab_index: source.tab_index,
            };
            let target = TabLocation {
                tab_index: target.tab_index,
                panel_id,
            };
            actions.move_tab(source, target);
        }),
    });

    let tab_props = {
        tab_title,
        tabIndex,
        isActive,
        tab_count,
        actions,
        isDragging,
        inner_ref: |el: ConnectableElement| {
            // hook inner tab ref to drag and drop
            connect_drag_ref(el);
            connect_drop_ref(el);
        },
        hidden: isDragging,
        highlight,
    };
    return <ToolbarTab {...tab_props} />;
}
```