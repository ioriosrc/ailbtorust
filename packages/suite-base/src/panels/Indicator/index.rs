```rust
use crate::{
    component::{Component, ComponentProps},
    hook::use_crash,
};
use lichtblick_suite_base::components::{
    CaptureErrorBoundary,
    Panel,
    PanelExtensionAdapter,
    create_sync_root,
    theme::ThemeProvider,
    types::PanelConfig,
};

struct IndicatorConfig {
    // Define your IndicatorConfig properties here
}

type SaveConfig<T> = fn(&mut T) -> ();

fn init_panel(crash: &dyn Fn(Box<dyn std::error::Error>) -> !, context: PanelExtensionContext) {
    create_sync_root(
        <CaptureErrorBoundary onError={crash}>
            <ThemeProvider is_dark>
                <Indicator context={context} />
            </ThemeProvider>
        </CaptureErrorBoundary>,
        context.panel_element,
    );
}

struct IndicatorLightPanelAdapterProps<'a> {
    config: &'a PanelConfig<IndicatorConfig>,
    save_config: fn(&mut &'a IndicatorConfig) -> (),
}

impl Component for IndicatorLightPanelAdapter<'_> {
    type Props = IndicatorLightPanelAdapterProps<'_>;

    fn render(&self, props: &Self::Props) -> Node {
        let crash = use_crash();
        let bound_init_panel = useMemo(
            move || init_panel.bind(None, crash),
            [crash],
        );

        <PanelExtensionAdapter
            config={props.config}
            save_config={props.save_config}
            init_panel={bound_init_panel}
            highest_supported_config_version={1}
        />
    }
}

pub fn Panel(props: PanelProps) -> Node {
    IndicatorLightPanelAdapter::render(
        props,
        &PanelConfig::<IndicatorConfig>::default(),
    )
}
```