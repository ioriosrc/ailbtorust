```rust
use mockall::prelude::*;
use lichtblick_suites_base::{panels::StateTransitions as LichtenblickStateTransitions, types::panels as LichtenblickPanels};
use light_test_builders::{basic_builder::BasicBuilder, state_transitions_config::StateTransitionConfig};

#[derive(Debug)]
struct OpenSiblingPanel {
    mock_open_sibling_panel: Mock<LichtenblickPanels>,
}

impl OpenSiblingPanel {
    fn new() -> Self {
        let mock = Mock::<LichtenblickPanels>::new();
        mock.expect_with(|_| LichtenblickPanels::OpenSibling {}).once().returning(|_| ());
        Self { mock_open_sibling_panel }
    }

    fn call(&self, topic_name: String) {
        self.mock_open_sibling_panel
            .expect_with(move |args| args.0 == "StateTransitions" && args.1.updateIfExists)
            .once()
            .withf(move |args| {
                args.2.siblingConfigCreator
                    == move |config| config.paths.contains(&TopicName(topic_name))
            })
            .returning(|_| ());
    }
}

#[derive(Debug)]
struct BasicBuilder {}

impl BasicBuilder {
    fn string() -> Self {
        Self {}
    }
}

#[test]
fn test_open_sibling_state_transitions_panel() {
    let mut mock_open_sibling_panel = OpenSiblingPanel::new();
    let topic_name = BasicBuilder.string().build();

    open_sibling_state_transitions_panel.call(topic_name);

    // Check that the mock was called with the correct arguments
    mock_open_sibling_panel.assert_call_count(1);
    mock_open_sibling_panel.assert_called_with({
        panel_type: "StateTransitions",
        updateIfExists: true,
        siblingConfigCreator: move |config| config.paths.contains(&TopicName(topic_name))
    });
}
```