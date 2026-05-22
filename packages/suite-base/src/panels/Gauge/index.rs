```rust
use crate::components::{CaptureErrorBoundary, Gauge};
use crate::config::GaugePanelAdapterProps;
use crate::hooks::use_crash;
use crate::panels::{create_sync_root, PanelExtensionContext};
use crate::theme::ThemeProvider;

fn init_panel(crash: Box<dyn Fn() -> ()>, context: &mut PanelExtensionContext) {
    create_sync_root(
        <CaptureErrorBoundary on_error={crash}>
            <ThemeProvider is_dark>
                <Gauge context={context} />
            </ThemeProvider>
        </CaptureErrorBoundary>,
        context.panel_element,
    );
}

fn GaugePanelAdapter(props: GaugePanelAdapterProps) -> Panel<ThemeProvider, CaptureErrorBoundary, Gauge> {
    let crash = use_crash();
    let bound_init_panel = Box::new(move || init_panel(Box::new(crash), props.context));
    Panel::new(
        ThemeProvider,
        CaptureErrorBoundary,
        Gauge,
        props.config,
        props.save_config,
        Some(bound_init_panel),
        1,
    )
}

GaugePanelAdapter.panel_type = "Gauge";
GaugePanelAdapter.default_config = {};
```