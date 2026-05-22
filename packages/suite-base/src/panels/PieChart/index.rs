```rust
use std::rc::Rc;

use crate::{
    components::{CaptureErrorBoundary, ThemeProvider},
    hooks::use_crash,
    types::{SaveConfig, PieChartConfig},
};

type InitPanelFn = fn(crash: Rc<dyn Fn(&str)>, context: PanelExtensionContext);

fn init_panel(crash: Rc<dyn Fn(&str)>, context: PanelExtensionContext) -> () {
    create_sync_root(
        <CaptureErrorBoundary onError={crash.clone()}>
            <ThemeProvider is_dark>
                <PieChart context={context} />
            </ThemeProvider>
        </CaptureErrorBoundary>,
        context.panel_element(),
    );
}

#[derive(Default)]
struct PieChartPanelAdapterConfig {}

type PanelType = &'static str;

fn main() {
    let crash = Rc::new(|msg| println!("Error: {}", msg));
    let config = PieChartPanelAdapterConfig {};

    // Your panel implementation here
}
```

Note that this is a simplified version of the original TypeScript/React code. The Rust code is a direct translation and assumes there's a corresponding `PieChart` component, `ThemeProvider`, and `create_sync_root` function available for use in the Rust ecosystem. Additionally, the panel type (`PanelType`) and default configuration are placeholders and should be replaced with actual implementations in a Rust project.