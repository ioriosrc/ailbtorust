```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn panic_hook(payload: *const u8, payload_len: usize);
}

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    console::error().with_str("Starting PanelLayout tests");
    // jsdom can't parse our @container CSS so we have to silence console.error for this test.
    panic_hook = Box::new(|payload: *const u8, payload_len: usize| {
        eprintln!("Panic hook called with payload {:?}", std::slice::from_raw_parts(payload, payload_len));
    });

    let render_a = async fn() -> Panel {
        render_with_type("a", |panel| panel)
    };
    let module_a = async fn() -> Module {
        Box::new(Panel::new(render_a(), { panel_type: "a", default_config: {} }))
    };

    let render_b = async fn() -> Panel {
        render_with_type("b", |panel| panel)
    };
    let module_b = async fn() -> Module {
        Box::new(Panel::new(render_b(), { panel_type: "b", default_config: {} }))
    };

    let render_c = async fn() -> Panel {
        render_with_type("c", |panel| panel)
    };
    let module_c = async fn() -> Module {
        Box::new(Panel::new(render_c(), { panel_type: "c", default_config: {} }))
    };

    let panels: Vec<PanelInfo> = vec![
        PanelInfo {
            title: "A".to_string(),
            type: "a".to_string(),
            module: module_a(),
        },
        PanelInfo {
            title: "B".to_string(),
            type: "b".to_string(),
            module: module_b(),
        },
        PanelInfo {
            title: "C".to_string(),
            type: "c".to_string(),
            module: module_c(),
        },
    ];

    let panel_catalog = MockPanelCatalog::new(panels);

    async fn render_with_type<P: Panel, M: Module>(
        type: &str,
        panel_factory: impl Fn(&Panel) -> P,
    ) -> P {
        panel_factory(panel::new(PanelsContext::get().unwrap(), { panel_type: type.to_string() }))
    }

    let rerender = async fn(
        layout: Layout,
        onChange: fn(),
    ) -> Result<(), JsValue> {
        let result = UnconnectedPanelLayout::new(layout, onChange);
        await Promise.resolve();
        Ok(())
    };

    let unmount = async fn(panel_layout: UnconnectedPanelLayout) -> Result<(), JsValue> {
        panel_layout.dispose();
        Ok(())
    };

    let (unmounted_panel_layout, layout, _) = setup_with_layout();

    // Each panel module should have only been loaded once
    assert_eq!(mock_module_a.calls(), 1);
    assert_eq!(mock_module_b.calls(), 1);
    assert_eq!(mock_module_c.calls(), 0);

    rerender(layout.change_split_percentage(50), |_| {});
    await Promise.resolve();
    // Each panel module should have only been loaded once
    assert_eq!(mock_module_a.calls(), 1);
    assert_eq!(mock_module_b.calls(), 1);
    assert_eq!(mock_module_c.calls(), 0);

    rerender(layout.change_split_percentage(40), |_| {});
    await Promise.resolve();
    // Each panel module should have only been loaded once; panels A and B should not render again
    assert_eq!(mock_module_a.calls(), 1);
    assert_eq!(mock_module_b.calls(), 1);
    assert_eq!(mock_module_c.calls(), 2);

    unmount(unmounted_panel_layout).await?;
    Ok(())
}
```