```rust
use crate::components::{
    MessagePathSyntax::{MessagePathInput, Select},
    PanelToolbar,
    Stack,
};
use crate::panels::RawMessagesCommon::{useStylesToolbar, PropsToolbar, RawMessagesVirtualPanelConfig};
use crate::suite_base::{
    constants::{PREV_MSG_METHOD, CUSTOM_METHOD},
};

fn ToolbarComponent(props: PropsToolbar) -> ReactElement {
    let (
        can_expand_all,
        diff_enabled,
        diff_method,
        diff_topic_path,
        on_diff_topic_path_change,
        on_toggle_diff,
        on_toggle_expand_all,
        on_topic_path_change,
        save_config,
        topic,
        topic_path,
    ) = props;

    let { classes, cx } = useStylesToolbar();

    let icon_button_props: ButtonProps = if diff_enabled {
        IconButtonProps::new()
            .color(Color::default())
            .icon(<DiffIcon fontSize="small" />)
            .title("Toggle diff")
            .onClick(move || on_toggle_diff())
            .size(ButtonSize::Small)
    } else {
        IconButtonProps::new()
            .color(Color::inherit())
            .icon(<DiffOutlinedIcon fontSize="small" />)
            .title("Toggle diff")
            .onClick(move || on_toggle_diff())
            .size(ButtonSize::Small)
    };

    let expand_button_props: ButtonProps = if can_expand_all {
        IconButtonProps::new()
            .color(Color::default())
            .icon(<UnfoldMoreIcon fontSize="small" />)
            .title("Expand all")
            .onClick(move || on_toggle_expand_all())
            .size(ButtonSize::Small)
    } else {
        IconButtonProps::new()
            .color(Color::default())
            .icon(<UnfoldLessIcon fontSize="small" />)
            .title("Collapse all")
            .onClick(move || on_toggle_expand_all())
            .size(ButtonSize::Small)
    };

    <PanelToolbar className={cx(classes.toolbar)}>
        <IconButton {...icon_button_props}>
            {diff_enabled && <DiffIcon fontSize="small" />}
            {!diff_enabled && <DiffOutlinedIcon fontSize="small" />}
        </IconButton>
        <IconButton {...expand_button_props} data-testid="expand-all">
            {can_expand_all && <UnfoldMoreIcon fontSize="small" />}
            {!can_expand_all && <UnfoldLessIcon fontSize="small" />}
        </IconButton>
        <Stack fullWidth paddingLeft={0.25}>
            <MessagePathInput
                index=0
                path={topic_path}
                onChange={on_topic_path_change}
                input_style={{ height: 20 }}
            />
        </Stack>
    </PanelToolbar>
    if diff_enabled {
        let select_props = SelectProps::new()
            .variant("filled")
            .size(ButtonSize::Small)
            .title("Diff method")
            .value(diff_method.clone())
            .menu_props(MenuListProps { dense: true })
            .onChange(move |event| {
                save_config({
                    diff_method: event.target.value as RawMessagesVirtualPanelConfig["diffMethod"],
                });
            });

        <Select {...select_props}>
            <MenuItem value={PREV_MSG_METHOD}>{PREV_MSG_METHOD}</MenuItem>
            <MenuItem value={CUSTOM_METHOD}>custom</MenuItem>
        </Select>
        if diff_method == CUSTOM_METHOD {
            let custom_input_props = MessagePathInputProps::new()
                .index(1)
                .path(diff_topic_path)
                .onChange(on_diff_topic_path_change)
                .props(topic.as_ref().map(|topic| topic.schema_name.clone()));

            <MessagePathInput {...custom_input_props} />
        }
    }
}
```