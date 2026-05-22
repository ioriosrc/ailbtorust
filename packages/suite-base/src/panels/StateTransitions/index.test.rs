```rust
use jest::{mock, expect, test};
use testing::builders::MessageEventBuilder;
use suite_base::components::{
    MessagePathSyntax::{useCachedGetMessagePathDataItems, useMessagesByPath},
    Panel,
    MessagePipeline,
    TimeBasedChart,
    PanelToolbar,
    StateTransitionsPanel,
};
use suite_base::panels::StateTransitions::{
    hooks::{useDecodedMessageRange, useStateTransitionsTime, useStateTransitionsData},
    hooks::useChartScalesAndBounds,
    hooks::useMessagePathDropConfig,
    hooks::usePanelSettings,
};
use suite_base::types::StateTransitionConfig;

#[test]
fn test_state_transitions_panel_rendering() {
    mock! { MessageEventsBuilder }
    mock! { BasicBuilder }

    fn build_message_and_data(path: &str) -> (MessageEvent, Vec<MessageDataItem>) {
        let topic = path.split(".").next().unwrap();
        (
            MessageEventBuilder.message_event(topic),
            vec![MessageDataItem::new(topic.to_string(), String::from("test"))],
        )
    }

    fn render_panel(config: Option<StateTransitionConfig> = None) -> Panel {
        let config = config.unwrap_or_default();
        let save_config = Box::new(|| ());
        StateTransitionsPanel::default()
            .set_config(config)
            .set_save_config(save_config)
            .build()
    }

    test! { "should render the panel" }
    expect(render_panel().get_element("time-based-chart")).to_be_not_null();
    expect(render_panel().get_element("path-legend")).to_be_not_null();

    test! { "should pass pathStrings to useMessagesByPath when no range data" }
    let topic_a = BasicBuilder.string();
    let topic_b = BasicBuilder.string();
    mock_decoded_messages(vec![vec![build_message_and_data(topic_a)]]);
    mock_use_messages_by_path(vec![topic_a, topic_b]);

    test! { "should pass empty array to useMessagesByPath when range data is active" }
    let topic = BasicBuilder.string();
    mock_decoded_messages(vec![vec![build_message_and_data(topic)]]);

    test! { "should pass pathStrings when decodedMessages has matching paths but empty arrays" }
    let topic = BasicBuilder.string();
    mock_decoded_messages(vec![vec![MessageDataItem::new(topic.to_string(), String::from("test"))]]);
    mock_use_messages_by_path(vec![topic]);

    test! { "should skip useMessagesByPath when any path has range data" }
    let topic_a = BasicBuilder.string();
    let topic_b = BasicBuilder.string();
    let decoded_messages: Vec<MessageDataItemsByPath> = vec![
        vec![build_message_and_data(topic_a)],
        vec![MessageDataItem::new(topic_b.to_string(), String::from("test"))],
    ];
    mock_decoded_messages(decoded_messages);

    mock_use_messages_by_path(vec![]);
}
```