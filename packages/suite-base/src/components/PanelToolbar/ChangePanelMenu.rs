```rust
use crate::components::PanelCatalog;
use crate::context::{MosaicActions, MosaicWindowActions};
use crate::hooks::useCurrentLayoutActions;

fn change_panel_menu({
    tab_id,
    anchor_el,
    onClose,
}: {
    tab_id: Option<&str>,
    anchor_el: Option<web_sys::Element>,
    onClose: fn(),
}) -> web_sys::HtmlElement {
    let panel_context = crate::context::PanelContext::get();
    let mosaic_actions = MosaicActions::get();
    let mosaic_window_actions = MosaicWindowActions::get();
    let swap_panel = use_current_layout_actions().swap_panel;

    let classes = useStyles();

    let handle_swap = move |id: Option<&str>| {
        if id.is_none() || panel_context.type == *id.unwrap() {
            onClose();
            return;
        }

        swap_panel({
            tab_id,
            original_id: id.unwrap_or_default(),
            type: panel_context.type.clone(),
            root: mosaic_actions.get_root().unwrap(),
            path: mosaic_window_actions.get_path().unwrap(),
            config: Default::default(),
        });
    };

    // https://github.com/mui/material-ui/issues/35287#issuecomment-1332327752
    let fix_mui_35287 = {} as { on_resize: Option<web_sys::EventTarget>, on_resize_capture: Option<web_sys::EventTarget> };

    web_sys::HtmlElement {
        inner_html: format!(
            r#"
              <div class="paper">
                <ClickAwayListener onClickAway={onClose}>
                  <PanelCatalog
                    mode="list"
                    isMenu
                    selectedPanelType={panel_context.type}
                    onPanelSelect={handle_swap}
                  />
                </ClickAwayListener>
              </div>
            "#
        ),
    }
}

fn main() {
    // Example usage
    let tab_id = Some("example_tab");
    let anchor_el = Some(web_sys::Element::new());
    let onClose = || println!("Panel menu closed");

    let element = change_panel_menu(tab_id, anchor_el, onClose);

    // You can now insert the `element` into your web page or handle it as needed
}
```

Please note that this code assumes you have a pre-existing Rust environment set up and familiarity with web_sys for handling DOM interactions. The specific implementation may vary depending on how you structure your Rust application and integrate it with your existing HTML/CSS/JS components.