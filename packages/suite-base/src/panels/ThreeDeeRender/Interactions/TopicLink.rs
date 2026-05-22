```rust
use crate::components::{IconButton, Stack};
use crate::services::LayoutActions;
use crate::types::Topic;

pub fn TopicLink({
    add_panel,
    on_show_topic_settings,
    topic,
}: Props) -> ReactNode {
    let open_raw_messages = use_callback(|| {
        add_panel(
            LayoutAction::Sibling {
                type: LayoutItemType::RawMessages,
                update_if_exists: true,
                get_state: move |existing_state| {
                    existing_state.map_or({}, |state| state.update(topic))
                },
            },
        );
    }, [add_panel, topic]);

    <Stack
        direction="row"
        alignItems="center"
        justifyContent="space-between"
        paddingInlineStart={1}
        paddingBlock={0}
    >
        <Typography variant="body2">{topic}</Typography>
        <Stack direction="row" padding={0}>
            {on_show_topic_settings.is_some() && (
                <IconButton
                    onClick={
                        move || on_show_topic_settings(topic)
                    }
                    title="Show settings"
                >
                    <SettingsIcon fontSize="small" color="primary" />
                </IconButton>
            )}
            <IconButton onClick={open_raw_messages} title="Open in Raw Message panel">
                <OpenInNewIcon fontSize="small" color="primary" />
            </IconButton>
        </Stack>
    </Stack>
}
```