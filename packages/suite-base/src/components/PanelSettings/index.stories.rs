```rust
use storybook::prelude::*;

mod PanelSettings {
    pub fn render() -> impl Into<&'static dyn Fn(Args) -> Box<dyn Component>> {
        provide_context::<CurrentLayoutProviderContext>(MockCurrentLayoutProvider {});
        provide_store::<PanelCatalogStateStore>("panel_catalog", MockPanelCatalog {});

        PanelSetup::new()
            .with_fixture(fixture())
            .with_omit_drag_and_drop(true)
    }
}

mod panels {
    pub const SAMPLE: &str = "Sample1!abc";
}

#[cfg(test)]
mod tests {
    use storybook_test::prelude::*;

    #[test]
    fn no_panel_selected() {
        render(|args| PanelSettings::render().args(args).view())
            .expect_template("components/PanelSettings")
            .expect_no_element(".panel-setup")
            .expect_no_component(":root > div.panel-settings")
    }

    #[test]
    fn panel_selected() {
        let mock_panel_catalog = MockPanelCatalog {};
        let fixture = fixture();
        let selected_panel_ids = vec![panels::SAMPLE];

        render(|args| PanelSettings::render()
            .args(args)
            .view_with_provider(panel_catalog, mock_panel_catalog)
            .fixture(fixture)
            .omit_drag_and_drop(true))
            .expect_template("components/PanelSettings")
            .expect_component(":root > div.panel-settings")
    }

    #[test]
    fn panel_selected_with_appbar() {
        let mock_panel_catalog = MockPanelCatalog {};
        let fixture = fixture();
        let selected_panel_ids = vec![panels::SAMPLE];
        let nodes = SettingsTreeNodes::new();

        render(|args| PanelSettings::render()
            .args(args)
            .view_with_provider(panel_catalog, mock_panel_catalog)
            .fixture(fixture)
            .omit_drag_and_drop(true))
            .expect_template("components/PanelSettings")
            .expect_component(":root > div.panel-settings")
    }

    #[test]
    fn panel_loading() {
        render(|args| PanelSettings::render().args(args).view_with_provider(MockPanelCatalog {}, MockPanelCatalog {}));
    }
}
```