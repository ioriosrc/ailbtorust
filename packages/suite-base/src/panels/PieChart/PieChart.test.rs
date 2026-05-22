```rust
// Import necessary crates for testing and Rust
use wasm_bindgen_test::wasm_bindgen_test;
use js_sys::Array;
use wasm_bindgen::{JsCast, JsValue};

#[wasm_bindgen_test]
async fn test_pie_chart() {
    // Initialize the mock context and settings tree hooks
    let mock_context: Box<dyn PanelExtensionContext> = Box::new(MockContext {});
    let mock_settings_tree = Box::new(MockSettingsTree {});

    // Mock the useLegendCount hook
    let legend_count = 10;
    let mut mock_legend_count = MockUseLegendCount { legend_count };
    JsValue::set_global(&JsValue::from("useLegendCount"), &JsValue::from(&mut mock_legend_count));

    // Mock the useSettingsTree hook
    let general_fields = [
        ("path", "Message path".into(), "messagepath".into()),
        ("title", "Title".into(), "Pie Chart".into()),
    ];
    let mut mock_settings_tree = MockUseSettingsTree { general_fields };
    JsValue::set_global(&JsValue::from("useSettingsTree"), &JsValue::from(&mut mock_settings_tree));

    // Render the PieChart component
    let rendered_component = render_to_string(format!("{{\"context\":{}}}", serde_json::to_string_pretty(&mock_context).unwrap()));

    // Check if the PieChart component is in the rendered output
    assert!(rendered_component.contains("Pie Chart"));

    // Check if the tooltip is formatted correctly
    let tooltip_content = format_tooltip(10, "Value");
    assert_eq!(tooltip_content[0], "10.00%");
    assert_eq!(tooltip_content[1], "Value");

    // Add more tests for other functionality as needed
}
```