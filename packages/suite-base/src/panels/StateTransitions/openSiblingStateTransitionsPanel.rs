```rust
use lichtblick_suite_base::panels::{OpenSiblingPanel, PanelConfig};

pub fn open_sibling_state_transitions_panel(
  open_sibling_panel: OpenSiblingPanel,
  topic_name: &str,
) {
  open_sibling_panel({
    panel_type: "StateTransitions",
    update_if_exists: true,
    sibling_config_creator: |config| {
      let existing_path = config
        .as_ref()
        .map(|cfg| cfg.as_any().downcast_ref::<StateTransitionConfig>())
        .and_then(|state_transition_config| {
          state_transition_config.paths.iter().find_map(|path| {
            if path.value == topic_name {
              Some(path)
            } else {
              None
            }
          })
        });
      if existing_path.is_some() {
        config
      } else {
        Box::new({
          let mut cfg = config.clone();
          cfg.as_any_mut().downcast_in_place::<StateTransitionConfig>().unwrap();
          let paths = cfg.paths.clone();
          cfg.paths.push(Path { value: topic_name, timestamp_method: "receive_time" });
          Box::new(cfg)
        })
      }
    },
  });
}
```