```rust
// Import necessary Rust libraries
use actix_web::{web, App, HttpServer};
use axum::routing::get;
use std::error::Error;

mod settings_tree_node_action_item;
mod node_actions_menu;
mod settingsTreeNode_builder;

#[derive(Debug, Clone)]
struct SettingsTreeNodeActionItem {
    label: String,
}

impl SettingsTreeNodeActionItem {
    fn new(label: &str) -> Self {
        Self { label }
    }
}

#[derive(Clone)]
struct NodeActionsMenuProps {
    actions: Vec<SettingsTreeNodeActionItem>,
    onSelectAction: Box<dyn Fn(&str)>,
}

async fn handle_node_actions_menu(req: HttpRequest, state: web::State<NodeActionsMenuProps>) -> Result<String, Error> {
    let selected_action = req.uri().query_pairs()
        .find_map(|(key, value)| if key == "action" { Some(value) } else { None })
        .ok_or("Action parameter is missing")?;

    state.on_select_action(selected_action);

    Ok(format!("Selected action: {}", selected_action))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let actions = vec![
        SettingsTreeNodeBuilder::node_action(),
        SettingsTreeNodeBuilder::node_divider(),
        SettingsTreeNodeBuilder::node_action(),
    ];

    let props = NodeActionsMenuProps {
        actions,
        onSelectAction: Box::new(|id| println!("Selected action ID: {}", id)),
    };

    HttpServer::new(move || App::new()
        .route("/", get().to(handle_node_actions_menu))
        .state(props))
        .bind("127.0.0.1:3000")?
        .run()
        .await
}
```