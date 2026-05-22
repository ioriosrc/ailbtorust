```rust
use crate::PanelSetup;
use crate::Publish;
use crate::PublishConfig;
use mockall::mock;
use mockall::predicate::eq;

mod test_publish {
    use super::*;

    #[tokio::test]
    async fn advanced_view_enabled() {
        let mut publish_builder = PublishBuilder::new();
        publish_builder.set_advanced_view(true);
        let config = publish_builder.build();

        let (ui, _) = setup(config);

        assert!(ui.query_selector("textarea").is_some());
    }

    #[tokio::test]
    async fn advanced_view_disabled() {
        let mut publish_builder = PublishBuilder::new();
        publish_builder.set_advanced_view(false);
        let config = publish_builder.build();

        let (ui, _) = setup(config);

        assert!(ui.query_selector("textarea").is_none());
    }

    #[tokio::test]
    async fn not_connected_to_data_source() {
        let mock_publisher = MockPublisher::new();
        mock_publisher.expect_publish().never();

        let mut publish_builder = PublishBuilder::new();
        publish_builder.set_topic_name("");
        publish_builder.set_datatype("");
        let config = publish_builder.build();

        let (ui, _) = setup(config, vec![PLAYER_CAPABILITIES::ADVERTISE]);

        assert!(ui.query_selector("p").contains_text("Connect to a data source that supports publishing"));
    }

    #[tokio::test]
    async fn connected_and_topic_and_datatype_not_configured() {
        let mut publish_builder = PublishBuilder::new();
        publish_builder.set_topic_name("");
        publish_builder.set_datatype("");

        let config = publish_builder.build();

        let (ui, _) = setup(config, vec![PLAYER_CAPABILITIES::ADVERTISE]);

        assert!(ui.query_selector("p").contains_text(
            "Configure a topic and message schema in the panel settings"
        ));
    }

    #[tokio::test]
    async fn all_publish_conditions_met() {
        let mock_publisher = MockPublisher::new();
        mock_publisher.expect_publish().returning(|_| Ok(()));

        let mut publish_builder = PublishBuilder::new();
        publish_builder.set_value("{\"data\": \"hello\"}");
        let config = publish_builder.build();

        let (ui, _) = setup(config, vec![PLAYER_CAPABILITIES::ADVERTISE]);

        assert!(ui.query_selector("button").is_not_disabled());

        user_event.click(ui.query_selector("button").unwrap());

        mock_publisher.assert_called_with(eq(&PublishConfig {
            button_text: config.button_text.to_string(),
            button_tooltip: config.button_tooltip.to_string(),
            advanced_view: config.advanced_view,
            value: config.value.to_string(),
        }));
    }

    #[tokio::test]
    async fn json_value_invalid() {
        let mut publish_builder = PublishBuilder::new();
        publish_builder.set_value("not-json");

        let config = publish_builder.build();

        let (ui, _) = setup(config, vec![PLAYER_CAPABILITIES::ADVERTISE]);

        assert!(ui.query_selector("p").contains_text(r#"/unexpected token"#));
    }
}
```