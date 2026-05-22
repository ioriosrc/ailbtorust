```rust
use testing::render_and_get_by_text;
use testing::fire_event;

// Given When
fn setup(info_override: Option<DiagnosticInfo>) -> (DiagnosticTable, DiagnosticStatusProps) {
    let info = DiagnosticsBuilder::info().build();
    let status = DiagnosticsBuilder::status_message(&DiagnosticsBuilder::key_value([
        ("key1".to_string(), "value1".to_string()),
        ("key2".to_string(), "0.5".to_string()), // Ensure the value is formatted as a float
    ])).build();

    let defaultProps: DiagnosticStatusProps = {
        info: info.clone(),
        numeric_precision: 2,
        onChange_split_fraction: Box::new(|_| {}),
        open_sibling_panel: Box::new(|_| {}),
        split_fraction: 0.5,
        topic_to_render: BasicBuilder::string().build(),
    };

    let mut table = DiagnosticTable::new(defaultProps);
    if let Some(info_override) = info_override {
        table.set_info(info_override);
    }

    (table, defaultProps)
}

// Then
#[test]
fn test_display_name_and_message() {
    let (mut table, _) = setup(None);

    assert!(render_and_get_by_text(&mut table, "Diagnostic Status"));
    assert!(render_and_get_by_text(&mut table, info.message));
}

#[test]
fn test_resize_handle_and_on_change_split_fraction() {
    let (mut table, defaultProps) = setup(None);

    let resize_handle = render_and_get_by_text(&mut table, "Resize handle");

    // Simulate mouse events
    table.on_mouse_down(resize_handle);
    table.on_mouse_move(window(), (100, 0));
    table.on_mouse_up(window());

    assert!(mock_on_change_split_fraction().called());
}

#[test]
fn test_key_value_pairs_with_numeric_precision() {
    let (mut table, defaultProps) = setup(None);

    // Ensure the values are formatted as a float
    let key2_value = DiagnosticsBuilder::key_value(("key2".to_string(), "0.5".to_string())).build();
    defaultProps.status.values.push(key2_value);

    assert!(render_and_get_by_text(&mut table, "value1"));
    assert!(render_and_get_by_text(&mut table, "value2"));
}

#[test]
fn test_open_in_plot_panel_and_state_transitions_panel() {
    let values = vec![
        DiagnosticsBuilder::key_value(("key1".to_string(), "value1".to_string())),
        DiagnosticsBuilder::key_value(("key2".to_string(), "0.5".to_string())), // Ensure the value is formatted as a float
    ];
    let status = DiagnosticsBuilder::status_message(&DiagnosticsBuilder::key_value([
        ("key1".to_string(), "value1".to_string()),
        ("key2".to_string(), "0.5".to_string()), // Ensure the value is formatted as a float
    ])).build();

    let info_override = DiagnosticsBuilder::info({
        status: Some(status),
    }).build();
    let (mut table, defaultProps) = setup(Some(info_override));

    assert!(render_and_get_by_text(&mut table, "Open in Plot panel"));
    assert!(render_and_get_by_text(&mut table, "Open in State Transitions panel"));

    // Simulate button clicks
    let plot_button = render_and_get_by_text(&mut table, "Open in Plot panel");
    fire_event::click(plot_button);

    assert!(mock_open_sibling_panel().called());

    let state_transitions_button = render_and_get_by_text(&mut table, "Open in State Transitions panel");
    fire_event::click(state_transitions_button);

    assert!(mock_open_sibling_panel().called());
}
```