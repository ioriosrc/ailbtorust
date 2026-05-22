```rust
use wasm_bindgen::prelude::*;
use wasm_bindgen_test_macro::wasm_bindgen_test;
use js_sys::{ArrayBufferView};
use js_sys::Reflect;
use wasm_bindgen::JsValue;

#[wasm_bindgen_test]
fn renders_empty_state_when_no_diagnostics_are_available() {
    let diagnostic_result = Map::new();
    let use_diagnostics = MockUseDiagnostics::new(diagnostic_result);

    let config = DiagnosticSummaryConfigBuilder::default()
        .build();

    let props = DiagnosticSummaryPropsBuilder::default()
        .with_config(config)
        .with_save_config(|config| {
            set_config(config);
        })
        .build();

    let ui = HtmlDocument::load(
        r#"
            <div style="width: 800px; height: 500px;">
                <PanelSetup>
                    <DiagnosticSummary {...props} />
                </PanelSetup>
            </div>
        "#,
    )
    .unwrap();

    assert!(ui.get_text_content().contains("waiting for messages"));
    assert!(ui.get_text_content().contains(&config.topic_to_render));
}

#[wasm_bindgen_test]
fn renders_diagnostics_and_pinned_items() {
    let hardware_id = BasicBuilder::string();
    let diagnostic_id = BasicBuilder::string();
    let diagnostic_info = DiagnosticsInfoBuilder::default()
        .build();

    let diagnostic_result = Map::new([
        [hardware_id, Map::new([[diagnostic_id, diagnostic_info]])],
    ]);
    let use_diagnostics = MockUseDiagnostics::new(diagnostic_result);

    setup(
        &config,
        &props,
        vec![PinnedItem {
            hardware_id: hardware_id.clone(),
            diagnostic_id: diagnostic_id.clone(),
        }],
    );

    assert!(ui.get_text_content().contains("diagnostic-summary-node-row-0"));
    assert!(ui.get_text_content().contains("diagnostic-summary-node-row-1"));
    assert!(ui.get_text_content().contains(&diagnostic_info.display_name));
}
```