```rust
use react::createElement;
use styled_components::{css, Theme};
use typed_dom::{div, span};

use crate::{
    components::panel_grid_card::PanelGridCard,
    types::{PanelInfo, PanelSelection},
};

#[derive(Debug)]
pub struct Props {
    filtered_panels: Vec<PanelInfo>,
    on_panel_select: fn(PanelSelection),
    search_query: Option<String>,
}

fn main() -> ReactElement<()> {
    let props = Props {
        filtered_panels: vec![
            // Sample panel info
        ],
        on_panel_select: |arg0| println!("Panel selected: {:?}", arg0),
        search_query: None,
    };

    create_element!(div, {
        class: css! {
            display: "grid !important";
            grid_template_columns: "repeat(auto-fill, minmax(240px, 1fr))";
            gap: theme.spacing(2);
        },
        children: props.filtered_panels
            .iter()
            .map(|panel_info| create_element!(PanelGridCard, {
                key: format!("{}-{}", panel_info.type, panel_info.title),
                panel: panel_info,
                search_query: props.search_query.clone(),
                on_click: move || {
                    props.on_panel_select(panel_info);
                },
            }))
            .collect::<Vec<_>>()
    })
}
```

Note that this is a simplified Rust version of the given TypeScript code, focusing solely on the rendering logic. The actual implementation would need to include more details such as styling, handling of errors, and potentially integrating with React's lifecycle methods or hooks.