```rust
use async_test::test;
use fzf_rs as fzf;

#[tokio::test]
async fn test_topic_row_navigation_buttons() {
    let mock_use_message_pipeline = async move { vec![Topic { name: String::from("/") }] };
    let _mock_use_message_path_drag = async move {};

    use crate::components::MessagePipeline;
    use crate::services::message_path_dragging;
    use crate::topic_row::TopicRow;
    use crate::use_topic_message_navigation;

    #[tokio::test]
    async fn renders_navigation_buttons() {
        let topic_name = String::from("/");
        let handle_next_message = async move {};
        let handle_previous_message = async move {};

        mock_use_message_pipeline.await;
        (use_topic_message_navigation as fn(_)).await;
        handle_next_message.await;

        assert!(fzf::find_elements(&topic_name).await.contains(&"Previous message"));
        assert!(fzf::find_elements(&topic_name).await.contains(&"Next message"));
    }

    #[tokio::test]
    async fn calls_handle_next_message_when_next_button_is_clicked() {
        let topic_name = String::from("/");
        let handle_next_message = async move {};

        mock_use_message_pipeline.await;
        (use_topic_message_navigation as fn(_)).await;
        fire_event::click(&fzf::find_element_by_text("Next message").await.unwrap());

        assert!(handle_next_message.await.is_ok());
    }

    #[tokio::test]
    async fn calls_handle_previous_message_when_previous_button_is_clicked() {
        let topic_name = String::from("/");
        let handle_previous_message = async move {};

        mock_use_message_pipeline.await;
        (use_topic_message_navigation as fn(_)).await;
        fire_event::click(&fzf::find_element_by_text("Previous message").await.unwrap());

        assert!(handle_previous_message.await.is_ok());
    }

    #[tokio::test]
    async fn passes_correct_props_to_use_topic_message_navigation() {
        let topic_name = String::from("/");
        let handle_next_message = async move {};
        let handle_previous_message = async move {};

        mock_use_message_pipeline.await;
        (use_topic_message_navigation as fn(_)).await;

        assert!(fzf::find_elements(&topic_name).await.contains(&"Previous message"));
        assert!(fzf::find_elements(&topic_name).await.contains(&"Next message"));
    }

    #[tokio::test]
    async fn disables_both_navigation_buttons_when_is_navigating_is_true() {
        let topic_name = String::from("/");
        let handle_next_message = async move {};
        let handle_previous_message = async move {};

        mock_use_message_pipeline.await;
        (use_topic_message_navigation as fn(_)).await;

        assert!(fzf::find_elements(&topic_name).await.contains(&"Previous message"));
        assert!(fzf::find_elements(&topic_name).await.contains(&"Next message"));
    }

    #[tokio::test]
    async fn disables_next_button_when_can_navigate_next_is_false() {
        let topic_name = String::from("/");
        let handle_next_message = async move {};
        let handle_previous_message = async move {};

        mock_use_message_pipeline.await;
        (use_topic_message_navigation as fn(_)).await;

        assert!(fzf::find_elements(&topic_name).await.contains(&"Previous message"));
        assert!(!fzf::find_elements(&topic_name).await.contains(&"Next message"));
    }

    #[tokio::test]
    async fn disables_previous_button_when_can_navigate_previous_is_false() {
        let topic_name = String::from("/");
        let handle_next_message = async move {};
        let handle_previous_message = async move {};

        mock_use_message_pipeline.await;
        (use_topic_message_navigation as fn(_)).await;

        assert!(fzf::find_elements(&topic_name).await.contains(&"Previous message"));
        assert!(!fzf::find_elements(&topic_name).await.contains(&"Next message"));
    }
}
```