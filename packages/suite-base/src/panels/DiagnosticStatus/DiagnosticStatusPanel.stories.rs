```rust
use lichtblick_suite_base::{ panels::DiagnosticStatus, storybook::PanelSetup, Fixture, LEVELS};
use std::collections::HashMap;

pub const fixture: Fixture = Fixture {
  topics: vec![
    {
      name: "/diagnostics",
      schema_name: "diagnostic_msgs/DiagnosticArray",
    },
  ],
  frame: Some({
    "/diagnostics": vec![
      DiagnosticStatus::new(
        LEVELS::OK,
        "name1",
        "hardware_id1",
        vec!["message 1", "message 2"],
      ),
      DiagnosticStatus::new(
        LEVELS::OK,
        "name2",
        "hardware_id1",
        vec!["message 3"],
        HashMap::from([
          ("key", "value"),
          ("key <b>with html</b>", "value <tt>with html</tt>"),
        ]),
      ),
      DiagnosticStatus::new(
        LEVELS::ERROR,
        "name1",
        "levels_id",
        vec!["error message"],
      ),
      DiagnosticStatus::new(
        LEVELS::OK,
        "name2",
        "levels_id",
        vec!["ok message"],
      ),
      DiagnosticStatus::new(
        LEVELS::STALE,
        "name3",
        "levels_id",
        vec!["stale message"],
      ),
      DiagnosticStatus::new(
        LEVELS::WARN,
        "name4",
        "levels_id",
        vec!["warn message"],
      ),
    ],
  }),
};

pub const Empty: PanelSetup = {
  fixture,
  storybook_title: "panels/DiagnosticStatus/DiagnosticStatusPanel",
};

pub const Default: PanelSetup = {
  fixture,
  override_config: Some({
    topic_to_render: "/diagnostics",
    selected_hardware_id: "levels_id",
  }),
};

pub const WithSettings: PanelSetup = {
  include_settings: true,
  fixture,
  override_config: Some({
    topic_to_render: "/diagnostics",
    selected_hardware_id: "hardware_id1",
    selected_name: "name2",
  }),
};

pub const SelectedHardwareIDOnly: PanelSetup = {
  fixture,
  override_config: Some({
    topic_to_render: "/diagnostics",
    selected_hardware_id: "hardware_id1",
    selected_name: None,
  }),
};

pub const SelectedName: PanelSetup = {
  fixture,
  override_config: Some({
    topic_to_render: "/diagnostics",
    selected_hardware_id: "hardware_id1",
    selected_name: "name2",
  }),
};

pub const MovedDivider: PanelSetup = {
  fixture,
  override_config: Some({
    topic_to_render: "/diagnostics",
    selected_hardware_id: "hardware_id1",
    selected_name: None,
    split_fraction: 0.25,
  }),
};

pub const OldDiagnosticsMarkedStale: PanelSetup = {
  include_settings: true,
  fixture: Fixture {
    ...fixture,
    active_data: Some({
      current_time: { sec: 10, nsec: 0 },
    }),
    frame: Some({
      "/diagnostics": vec![
        DiagnosticStatus::new(
          LEVELS::OK,
          "name1",
          "timeout_id",
          vec!["2 secs"],
          HashMap::from([
            ("stamp", { sec: 2, nsec: 0 }),
          ]),
        ),
        DiagnosticStatus::new(
          LEVELS::OK,
          "name2",
          "timeout_id",
          vec!["4 secs"],
          HashMap::from([
            ("stamp", { sec: 4, nsec: 0 }),
          ]),
        ),
        DiagnosticStatus::new(
          LEVELS::OK,
          "name3",
          "timeout_id",
          vec!["6 secs"],
          HashMap::from([
            ("stamp", { sec: 6, nsec: 0 }),
          ]),
        ),
      ],
    }),
  },
};
```