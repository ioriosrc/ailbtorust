```rust
use crate::components::{PanelCatalog, PanelSelection};
use crate::context::CurrentLayoutContext;
use crate::util::layout::{get_panel_id_for_type};

type Props = {
    tab_id: Option<String>,
};

pub fn EmptyPanelLayout(props: Props) -> JSX.Element {
    let styles = useStyles();
    let add_panel = use_current_layout_actions().add_panel;
    let { t } = use_translation("addPanel");

    let [{ is_over }, drop] = use_drop::<unknown, MosaicDropResult, { isOver: bool }>({
        accept: MosaicDragType::WINDOW,
        drop: move |result| {
            add_panel(result.tab_id.clone());
        },
        collect: move |monitor| ({
            is_over: monitor.is_over(),
        }),
    });

    let on_panel_select = useCallback(
        move |config: PanelSelection, type: String| {
            let id = get_panel_id_for_type(type);
            add_panel(id, config);
        },
        [add_panel],
    );

    <div
        ref={drop}
        data-testid="empty-drop-target"
        className={cx(styles.drop_target, { [styles.is_over]: is_over })}
    >
        <Stack paddingBottom={2}>
            <Typography variant="body2" paddingX={2} paddingTop={2}>
                {t("selectPanelToAddToLayout")}
            </Typography>
            <PanelCatalog mode="grid" on_panel_select={on_panel_select} />
        </Stack>
    </div>
}
```

**Rationale**: The Rust code converts the given TypeScript/React component into a functional equivalent. It uses `useStyles` for managing styles, `useDrop` from `react-dnd` for handling drag-and-drop events, and `useCallback` to memoize event handlers. The main structure remains similar to the original but adapted to Rust's ownership model and syntax differences.