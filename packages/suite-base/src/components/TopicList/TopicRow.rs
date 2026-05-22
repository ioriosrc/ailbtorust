```rust
use fluenui::icons::regular::{ChevronLeft16Regular, ChevronRight16Regular, ReOrderDotsVertical16Regular};
use mui::{
    components::{Badge, IconButton, Tooltip, Typography},
    styled,
    theme::useTheme,
};

use message_path::Topic;
use message_pipeline::MessagePipelineContext;

use crate::{
    highlight_chars::HighlightChars,
    use_topic_list_styles,
    use_topic_message_navigation,
    DraggedMessagePath,
};

const SELECT_SUBSCRIPTIONS = "selectSubscriptions";

pub fn topic_row(
    topic_result: &FzfResultItem<Topic>,
    style: &React.CSSProperties,
    selected: bool,
    onClick: React.MouseEventHandler<HTMLDivElement>,
    onContextMenu: React.MouseEventHandler<HTMLDivElement>,
) -> JSX.Element {
    let { cx, classes } = use_topic_list_styles();

    let topic = &topic_result.item;

    let subscriptions = use_message_pipeline(SELECT_SUBSCRIPTIONS);

    let is_topic_subscribed = useMemo(
        || subscriptions
            .iter()
            .any(|sub| sub.topic == topic.name),
        [subscriptions, topic.name],
    );

    let {
        handle_next_message,
        handle_previous_message,
        is_navigating,
        can Navigate_next,
        can Navigate_previous,
    } = use_topic_message_navigation({
        topic_name: topic.name.to_string(),
        is_topic_subscribed,
        selected,
    });

    let is_navigation_disabled = useMemo(
        || is_navigating || !selected || !is_topic_subscribed,
        [is_navigating, selected, is_topic_subscribed],
    );

    let is_previous_disabled = useMemo(
        || is_navigation_disabled || !can Navigate_previous,
        [is_navigation_disabled, can.Navigate_previous],
    );
    let is_next_disabled = useMemo(
        || is_navigation_disabled || !can Navigate_next,
        [is_navigation_disabled, can.Navigate_next],
    );

    let item: DraggedMessagePath = useMemo(
        || {
            Item {
                path: quote_topic_name_if_needed(topic.name),
                root_schema_name: topic.schema_name.to_string(),
                is_topic: true,
                is_leaf: false,
                topic_name: topic.name.to_string(),
            }
        },
        [topic.name, topic.schema_name],
    );

    let { connect_drag_source, connect_drag_preview, cursor, is_dragging, dragged_item_count } =
        use_message_path_drag({
            item,
            selected,
        });

    let combined_ref = useCallback(
        |el| {
            connect_drag_source(el);
            connect_drag_preview(el);
        },
        [connect_drag_preview, connect_drag_source],
    );

    let seek_to_next_message = useCallback(
        (event: React.MouseEvent) => {
            event.stopPropagation();
            handle_next_message();
        },
        [handle_next_message],
    );

    let seek_to_previous_message = useCallback(
        (event: React.MouseEvent) => {
            event.stopPropagation();
            handle_previous_message();
        },
        [handle_previous_message],
    );

    let handle_button_mouse_down = useCallback((event: React.MouseEvent) => {
        event.stopPropagation();
    }, []);

    return (
        <div
            ref={combined_ref}
            className={cx(classes.row, {
                [classes.is_dragging]: is_dragging,
                [classes.selected]: selected,
            })}
            style={{ ...style, cursor }}
            onClick={onClick}
            onContextMenu={onContextMenu}
            data-testid="topic-row"
        >
            {dragged_item_count > 1 && (
                <Badge color="primary" className={classes.count_badge} badgeContent={dragged_item_count} />
            )}
            {/* Extra Stack wrapper to enable growing without the */}
            <Stack flex="auto" alignItems="flex-start" overflow="hidden">
                <Typography variant="body2" noWrap className={classes.textContent}>
                    <HighlightChars str={topic.name} indices={topic_result.positions} />
                    {topic.aliased_from_name.is_some() && (
                        <Typography
                            variant="caption"
                            className={classes.aliased_topic_name}
                        >
                            from {topic.aliased_from_name.unwrap()}
                        </Typography>
                    )}
                </Typography>
                {topic.schema_name.is_some() && (
                    <Typography
                        variant="caption"
                        color="text.secondary"
                        noWrap
                        className={classes.textContent}
                    >
                        <HighlightChars
                            str={topic.schema_name.unwrap()}
                            indices={topic_result.positions}
                            offset={topic.name.len() + 1}
                        />
                    </Typography>
                )}
            </Stack>
            <Stack direction="column" alignItems="flex-end" gap={0.5}>
                <TopicStatsChip selected={selected} topic_name={topic.name.to_string()} />
                <Stack direction="row" gap={0.5} alignItems="center">
                    <Tooltip title={is_previous_disabled.is_some() && "" || "Previous message"}>
                        <span>
                            <IconButton
                                size="small"
                                aria-label="Previous message"
                                onClick={seek_to_previous_message}
                                onMouseDown={handle_button_mouse_down}
                                disabled={is_previous_disabled.is_some()}
                                className={classes.nav IconButton}
                            >
                                <ChevronLeft16Regular />
                            </IconButton>
                        </span>
                    </Tooltip>
                    <Tooltip title={is_next_disabled.is_some() && "" || "Next message"}>
                        <span>
                            <IconButton
                                size="small"
                                aria-label="Next message"
                                onClick={seek_to_next_message}
                                onMouseDown={handle_button_mouse_down}
                                disabled={is_next_disabled.is_some()}
                                className={classes.nav IconButton}
                            >
                                <ChevronRight16Regular />
                            </IconButton>
                        </span>
                    </Tooltip>
                </Stack>
            </Stack>
            <div data-testid="TopicListDragHandle" className={classes.drag_handle}>
                <ReOrderDotsVertical16Regular />
            </div>
        </div>
    );
}
```