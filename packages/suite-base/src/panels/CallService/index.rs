```rust
use crate::components::{CaptureErrorBoundary, CallService, Config};
use crate::hooks::use_crash;
use crate::panels::{create_sync_root, PanelExtensionAdapter};

fn init_panel(crash: &crate::crash::Crash, context: PanelExtensionContext) -> Box<dyn Panel> {
    create_sync_root(
        <CaptureErrorBoundary onError={crash}>
            <CallService context={context} />
        </CaptureErrorBoundary>,
        context.panel_element,
    )
}

#[derive(Debug)]
pub struct Props<'a> {
    config: Config,
    save_config: fn(&mut Config),
}

fn call_service_panel_adapter(props: &'a Props) -> Box<dyn Panel> {
    let crash = use_crash();

    let bound_init_panel = move |crash: &crate::crash::Crash, context: PanelExtensionContext| {
        init_panel(crash, context)
    };

    PanelExtensionAdapter {
        config: props.config,
        save_config: props.save_config,
        init_panel: bound_init_panel,
        highest_supported_config_version: 1,
    }
}

impl<'a> Panel for call_service_panel_adapter<'a> {
    fn panel_type(&self) -> &'static str {
        "CallService"
    }

    fn default_config(&self) -> Config {
        Config::default()
    }
}
```